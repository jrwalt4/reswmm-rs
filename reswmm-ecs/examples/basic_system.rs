//! The basic working principle of how Systems communicate
//! with eachother and wait for Parameters to be resolved.

use std::{
    any::TypeId,
    cell::UnsafeCell,
    collections::HashMap,
    fmt::Debug,
    mem::transmute,
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex as StdMutex,
    },
};

use futures::{
    executor::block_on,
    future::{poll_fn, Future},
    join,
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

type WatcherMap = HashMap<TypeId, Vec<Waker>>;

#[derive(Default)]
struct ComponentStore {
    length: ComponentAccess<Length>,
    width: ComponentAccess<Width>,
    area: ComponentAccess<Area>,
    watchers: Arc<StdMutex<WatcherMap>>,
}

struct ComponentAccess<C> {
    complete: AtomicBool,
    value: UnsafeCell<Option<C>>,
}

/// An RAII reference, but for this example it's just a reference.
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

impl ComponentStore {
    fn new() -> Self {
        Self::default()
    }

    fn get_component<C: Component>(
        &self,
    ) -> Option<impl Future<Output = Option<ComponentRef<'_, C>>>> {
        let comp = self.get_component_inner::<C>()?;

        let fut = poll_fn(
            |cx: &mut Context<'_>| -> Poll<Option<ComponentRef<'_, C>>> {
                if comp.complete.load(Ordering::Acquire) {
                    let result = unsafe { (*comp.value.get()).as_ref().map(Into::into) };
                    return Poll::Ready(result);
                }
                match self.watchers.lock() {
                    Ok(mut lock) => {
                        let waker = cx.waker();
                        lock.entry(TypeId::of::<C>())
                            .and_modify(|v| {
                                if !v.iter().any(|w| w.will_wake(waker)) {
                                    v.push(waker.clone())
                                }
                            })
                            .or_insert_with(|| vec![waker.clone()]);
                        Poll::Pending
                    }
                    Err(_) => Poll::Ready(None),
                }
            },
        );

        Some(fut)
    }

    /// Set the value to the Option provided, returning either the previous value if
    /// successful or the provided value if unsuccessful
    fn set_component<C: Component>(&self, mut other: Option<C>) -> Result<Option<C>, Option<C>> {
        self.get_component_inner::<C>()
            .and_then(|comp| {
                match comp.complete.load(Ordering::Acquire) {
                    true => None,
                    false => {
                        let inner_opt: &mut Option<C> = unsafe { &mut *comp.value.get() };
                        let orig = inner_opt.take();
                        *inner_opt = other.take();
                        // mark as `complete` and alert watchers
                        // in reality should use a `compare_exchange` to make sure
                        // noone else tried to set after our check, but in this example we know
                        // there's only one system writing to each component.
                        comp.complete.store(true, Ordering::Release);
                        if let Some(wakers) =
                            self.watchers.lock().unwrap().get_mut(&TypeId::of::<C>())
                        {
                            for waker in wakers.drain(..) {
                                waker.wake();
                            }
                        }

                        // return result
                        Some(orig)
                    }
                }
            })
            .ok_or_else(|| other.take())
    }

    fn get_component_inner<C: Component>(&self) -> Option<&ComponentAccess<C>> {
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

impl<C> Default for ComponentAccess<C> {
    fn default() -> Self {
        Self {
            complete: AtomicBool::default(),
            value: UnsafeCell::new(None),
        }
    }
}

async fn solve_length(store: &ComponentStore) -> Result<Option<Length>, Option<Length>> {
    println!("Setting length");
    let result = store.set_component::<Length>(Length(2.0).into());
    println!("Set length");
    result
}

async fn solve_width(store: &ComponentStore) -> Result<Option<Width>, Option<Width>> {
    println!("Setting width");
    let result = store.set_component::<Width>(Width(2.0).into());
    println!("Set width");
    result
}

async fn solve_area(store: &ComponentStore) -> Result<Option<Area>, Option<Area>> {
    println!("Getting length and width");
    let (length, width) = join!(
        store.get_component::<Length>().unwrap(),
        store.get_component::<Width>().unwrap()
    );
    println!("Calculating area");
    let result = store.set_component::<Area>(Area(length.unwrap().0 * width.unwrap().0).into());
    println!("Setting area");
    result
}

fn main() {
    let store = ComponentStore::new();

    // solve out of order to show that area will wait for length and width to be set.
    let _ = block_on(async {
        join!(
            solve_area(&store),
            solve_length(&store),
            solve_width(&store),
        )
    });

    // now get the results
    let (length, width, area) = block_on(async {
        join!(
            store.get_component::<Length>().unwrap(),
            store.get_component::<Width>().unwrap(),
            store.get_component::<Area>().unwrap()
        )
    });
    println!("Length: {length:?}, Width: {width:?}, Area: {area:?}");
}
