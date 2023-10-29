//! The next level of abstraction where systems
//! iterate through entities using streams.

use std::{
    any::TypeId,
    cell::UnsafeCell,
    fmt::Debug,
    mem::{transmute, MaybeUninit},
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex as StdMutex,
    },
};

use futures::{
    executor::block_on,
    join,
    stream::{FusedStream, Stream, StreamExt},
    task::*,
};

trait Component: Send + Sync + 'static {}

#[derive(Debug)]
struct Length(f32);
impl Component for Length {}

#[derive(Debug)]
struct Width(f32);
impl Component for Width {}

#[derive(Debug)]
struct Area(f32);
impl Component for Area {}

/// A type mimicing how Queries work over Archetypes and
/// iterate over all Archetypes with a certain Component.
type Query<C> = (Vec<MaybeUninit<C>>, Vec<MaybeUninit<C>>);

struct ComponentAccess<C> {
    complete: AtomicBool,
    watchers: Arc<StdMutex<Vec<Waker>>>,
    value: UnsafeCell<Query<C>>,
}

impl<C> Default for ComponentAccess<C> {
    fn default() -> Self {
        Self::with_capacity((3, 3))
    }
}

impl<C> ComponentAccess<C> {
    fn with_capacity(caps: (usize, usize)) -> Self {
        let mut v1 = Vec::with_capacity(caps.0);
        v1.resize_with(caps.0, MaybeUninit::uninit);
        let mut v2 = Vec::with_capacity(caps.1);
        v2.resize_with(caps.0, MaybeUninit::uninit);
        Self {
            complete: AtomicBool::default(),
            watchers: Default::default(),
            value: UnsafeCell::new((v1, v2)),
        }
    }
}

/// An RAII reference to a completed value.
struct ComponentRef<'a, C> {
    component: &'a C,
}

impl<'a, C: Component> From<&'a C> for ComponentRef<'a, C> {
    fn from(value: &'a C) -> Self {
        Self { component: value }
    }
}

impl<C> Deref for ComponentRef<'_, C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<C: Debug> Debug for ComponentRef<'_, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.component.fmt(f)
    }
}

struct ComponentRefMut<'a, C> {
    value: &'a mut MaybeUninit<C>,
    is_init: bool,
}

impl<'a, C: 'static> ComponentRefMut<'a, C> {
    fn uninit(value: &'a mut MaybeUninit<C>) -> Self {
        Self {
            value,
            is_init: false,
        }
    }

    #[allow(dead_code)]
    fn init(value: &'a mut MaybeUninit<C>) -> Self {
        Self {
            value,
            is_init: true,
        }
    }

    fn set(&mut self, value: C) {
        if self.is_init {
            unsafe {
                self.value.assume_init_drop();
            }
        }
        self.value.write(value);
        self.is_init = true;
    }
}

type SystemResult<C> = Result<C, String>;

#[derive(Default)]
struct ComponentStore {
    length: ComponentAccess<Length>,
    width: ComponentAccess<Width>,
    area: ComponentAccess<Area>,
}

impl ComponentStore {
    fn new() -> Self {
        Self::default()
    }

    fn query_component<C: Component>(&self) -> impl Stream<Item = ComponentRef<'_, C>> {
        use std::iter::Chain;
        use std::slice::Iter;

        enum QueryStream<'s, C> {
            Waiting(&'s ComponentAccess<C>),
            Streaming(Chain<Iter<'s, MaybeUninit<C>>, Iter<'s, MaybeUninit<C>>>),
            Done,
        }

        impl<'s, C> Stream for QueryStream<'s, C> {
            type Item = &'s MaybeUninit<C>;
            fn poll_next(
                mut self: std::pin::Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                match self.deref_mut() {
                    Self::Waiting(ca) => {
                        if ca.complete.load(Ordering::Acquire) {
                            let comp = unsafe { &*ca.value.get() };
                            let mut iter = comp.0.iter().chain(&comp.1);
                            let first = iter.next();
                            *self = Self::Streaming(iter);
                            return Poll::Ready(first);
                        }
                        match ca.watchers.lock() {
                            Ok(mut v) => {
                                v.push(cx.waker().clone());
                                Poll::Pending
                            }
                            Err(_) => Poll::Ready(None),
                        }
                    }
                    Self::Streaming(iter) => {
                        let next = iter.next();
                        if next.is_none() {
                            *self = Self::Done;
                        }
                        Poll::Ready(next)
                    }
                    Self::Done => Poll::Ready(None),
                }
            }
        }

        impl<'s, C> FusedStream for QueryStream<'s, C> {
            fn is_terminated(&self) -> bool {
                matches!(self, Self::Done)
            }
        }

        let qs = match self.query_component_inner::<C>() {
            Some(ca) => QueryStream::Waiting(ca),
            None => QueryStream::Done,
        };
        qs.map(|munit| unsafe { ComponentRef::from(munit.assume_init_ref()) })
    }

    /// Set the value of the query, returning either a reference to the new value
    /// or an error.
    fn query_component_mut<C: Component>(&self) -> impl Iterator<Item = ComponentRefMut<'_, C>> {
        let ca = self.query_component_inner::<C>().unwrap();

        let inner: &mut Query<C> = unsafe { &mut *ca.value.get() };
        let mut iter = inner
            .0
            .iter_mut()
            .chain(inner.1.iter_mut())
            .map(|munit| ComponentRefMut::uninit(munit));

        std::iter::from_fn(move || -> Option<ComponentRefMut<'_, C>> {
            let next = iter.next();

            if next.is_none() {
                // mark as `complete` and alert watchers
                // in reality should use a `compare_exchange` to make sure
                // noone else tried to set after our check, but in this example we know
                // there's only one system writing to each component.
                ca.complete.store(true, Ordering::Release);
                if let Ok(mut wakers) = ca.watchers.lock() {
                    for waker in wakers.drain(..) {
                        waker.wake();
                    }
                }
            }
            next
        })
    }

    fn query_component_inner<C: Component>(&self) -> Option<&ComponentAccess<C>> {
        #![allow(non_snake_case)]
        // Simulate the Archetype pattern with hard-coded type lookup
        let LENGTH_ID: TypeId = TypeId::of::<Length>();
        let WIDTH_ID: TypeId = TypeId::of::<Width>();
        let AREA_ID: TypeId = TypeId::of::<Area>();
        let c_id = TypeId::of::<C>();
        if c_id == LENGTH_ID {
            unsafe {
                return Some(transmute(&self.length));
            }
        }
        if c_id == WIDTH_ID {
            unsafe {
                return Some(transmute(&self.width));
            }
        }
        if c_id == AREA_ID {
            unsafe {
                return Some(transmute(&self.area));
            }
        }
        None
    }
}

async fn solve_length(store: &ComponentStore) -> SystemResult<()> {
    println!("Setting length");
    for mut l in store.query_component_mut::<Length>() {
        l.set(Length(2.0));
    }
    println!("Set length");
    Ok(())
}

async fn solve_width(store: &ComponentStore) -> SystemResult<()> {
    println!("Setting width");
    for mut w in store.query_component_mut::<Width>() {
        w.set(Width(2.0));
    }
    println!("Set width");
    Ok(())
}

async fn solve_area(store: &ComponentStore) -> SystemResult<()> {
    println!("Getting length and width");
    let length_query = store.query_component::<Length>();
    let width_query = store.query_component::<Width>();
    println!("Calculating area");
    let _area_stream = length_query
        .zip(width_query)
        .zip(futures::stream::iter(store.query_component_mut::<Area>()))
        .for_each(|((length, width), mut area)| {
            area.set(Area(length.0 * width.0));
            futures::future::ready(())
        })
        .await;
    Ok(())
}

fn main() {
    let store = ComponentStore::new();

    // solve out of order to show that area will wait for length and width to be set.
    let (_area, _length, _width) = block_on(async {
        join!(
            solve_area(&store),
            solve_length(&store),
            solve_width(&store),
        )
    });

    async fn print_results<'a>(
        ((length, width), area): (
            (ComponentRef<'a, Length>, ComponentRef<'a, Width>),
            ComponentRef<'a, Area>,
        ),
    ) {
        println!("Length: {length:?}, Width: {width:?}, Area: {area:?}");
    }

    block_on(
        store
            .query_component::<Length>()
            .zip(store.query_component::<Width>())
            .zip(store.query_component::<Area>())
            .for_each(print_results),
    );
}
