use std::{
    alloc::{alloc, dealloc, Layout},
    any::{type_name, TypeId},
    collections::HashMap,
    mem::needs_drop,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::entity::{Entity, EntityId};

/// A type that can be stored as part of an [`Entity`]. 
pub trait Component: Send + Sync + 'static {}

impl Component for () {}

/// A group of [`Component`]s
pub(crate) trait Bundle: Send + Sync + 'static {
    fn info() -> Vec<ComponentInfo>;

    fn id() -> Vec<ComponentId> {
        Self::info().into_iter().map(| info | info.id).collect()
    }
}

impl<C0: Component> Bundle for (C0,) {
    fn info() -> Vec<ComponentInfo> {
        vec![ComponentInfo::of::<C0>()]
    }
}

impl<C0: Component, C1: Component> Bundle for (C0, C1) {
    fn info() -> Vec<ComponentInfo> {
        vec![ComponentInfo::of::<C0>(), ComponentInfo::of::<C1>()]
    }
}

/// Unique identifier of a Component (alias for [`TypeId`]).
pub type ComponentId = TypeId;

/// Metadata used to create, access, and drop [`Component`]s.
pub(crate) struct ComponentInfo {
    id: ComponentId,
    layout: Layout,
    drop: Option<unsafe fn(*mut u8)>,
    #[cfg(debug_assertions)]
    name: &'static str,
}

impl ComponentInfo {
    #[inline]
    pub(crate) fn of<C: Component>() -> Self {
        unsafe fn drop_internal<T>(p: *mut u8) {
            p.cast::<T>().drop_in_place();
        }
        Self {
            id: TypeId::of::<C>(),
            layout: Layout::new::<C>(),
            drop: needs_drop::<C>().then_some(drop_internal::<C> as _),
            #[cfg(debug_assertions)]
            name: type_name::<C>(),
        }
    }
}

/// A block of [`Component`] data.
struct ComponentColumn {
    info: ComponentInfo,
    data: NonNull<u8>,
}

impl ComponentColumn {
    fn new(info: ComponentInfo) -> Self {
        let data = unsafe {
            // An aligned 'dangling' pointer.
            // Replace with `Layout::dangling` when it stabilizes.
            NonNull::new_unchecked(info.layout.align() as *mut u8)
        };
        Self { info, data }
    }

    fn with_capacity(info: ComponentInfo, capacity: usize) -> Option<Self> {
        if capacity == 0 {
            return Some(Self::new(info));
        }
        let mem = unsafe {
            alloc(Layout::from_size_align(info.layout.size() * capacity, info.layout.align()).ok()?)
        };
        Some(Self {
            info,
            data: NonNull::new(mem)?,
        })
    }

    fn as_ptr<C: Component>(&self) -> *const C {
        assert_eq!(self.info.id, TypeId::of::<C>());
        self.data.cast::<C>().as_ptr() as *const C
    }

    fn as_ptr_mut<C: Component>(&mut self) -> *mut C {
        self.as_ptr::<C>() as *mut C
    }
}

/// A unique identifier for an [`Archetype`]. ID's are generated by the [`ArchetypeManager`].
#[repr(transparent)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub(crate) struct ArchetypeId(u32);

/// A group of [`ComponentColumn`]s for entities that have the exact same set of Components.
pub struct Archetype {
    id: ArchetypeId,

    /// The entities stored in this Archetype.
    entities: Vec<EntityId>,

    /// Allocated space in each column.
    capacity: usize,

    /// Index of Component columns. Must be in same order as [`Archetype::components`].
    component_ids: Vec<ComponentId>,

    /// Actual Column data, must be in same order as [`Archetype::component_ids`].
    components: Vec<ComponentColumn>,
}

impl Archetype {

    fn from_info_with_capacity(
        id: ArchetypeId,
        components: impl IntoIterator<Item = ComponentInfo>,
        capacity: usize,
    ) -> Self {
        let mut component_info = components.into_iter().collect::<Vec<ComponentInfo>>();
        component_info.sort_by_key(|info| info.id);
        let (component_ids, components) = component_info
            .into_iter()
            .map(|info| {
                let id = info.id;
                let name = info.name;
                let col_opt = ComponentColumn::with_capacity(info, capacity);
                #[cfg(debug_assertions)]
                let col = col_opt.expect(name);
                #[cfg(not(debug_assertions))]
                let col = col_opt.unwrap();
                (id, col)
            })
            .unzip();
        Self {
            id,
            entities: Vec::new(),
            capacity,
            component_ids,
            components,
        }
    }

    fn from_info(id: ArchetypeId, components: impl IntoIterator<Item = ComponentInfo>) -> Self {
        Self::from_info_with_capacity(id, components, 0)
    }

    fn component_ids(&self) -> &[ComponentId] {
        &self.component_ids
    }

    fn len(&self) -> usize {
        self.entities.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn get_column<C: Component>(&self) -> Option<&ComponentColumn> {
        self.get_column_by_id(TypeId::of::<C>())
    }

    fn get_column_mut<C: Component>(&mut self) -> Option<&mut ComponentColumn> {
        self.get_column_index::<C>()
            .ok()
            .map(|index| &mut self.components[index])
    }

    fn get_column_by_id(&self, id: ComponentId) -> Option<&ComponentColumn> {
        self.get_column_index_by_id(id)
            .ok()
            .map(|index| &self.components[index])
    }

    fn get_column_index<C: Component>(&self) -> Result<usize, usize> {
        self.get_column_index_by_id(TypeId::of::<C>())
    }

    fn get_column_index_by_id(&self, id: ComponentId) -> Result<usize, usize> {
        self.components
            .binary_search_by_key(&id, |data| data.info.id)
    }

    // Safety: May not actually be unsafe since we are keeping track of the
    // length and ensuring that C matches what is stored in the corresponding Column
    unsafe fn get_column_as_slice<C: Component>(&self) -> Option<&[C]> {
        self.get_column::<C>()
            .map(|col| std::slice::from_raw_parts(col.as_ptr(), self.len()))
    }

    fn insert(&mut self, entity: Entity) -> ArchetypeRowMut<'_> {
        let new_index = self.entities.len();
        self.entities.push(entity.id());
        ArchetypeRowMut::new(self, new_index)
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        let size = self.len();
        for col in &mut self.components {
            // Drop each of the entries as needed
            if let Some(d) = col.info.drop {
                let mut data = col.data.as_ptr();
                for _i in 0..size {
                    unsafe {
                        d(data);
                        data = data.add(col.info.layout.size());
                    }
                }
            }
            // release memory
            if col.info.layout.size() > 0 {
                unsafe { 
                    dealloc(
                        col.data.as_mut(), 
                        Layout::from_size_align_unchecked(
                            col.info.layout.size() * self.capacity, 
                            col.info.layout.align()
                        )
                    );
                }
            }
        }
    }
}

pub(crate) struct ArchetypeRow<'a> {
    archetype: &'a Archetype,
    index: usize,
}

impl<'r> ArchetypeRow<'r> {
    fn new<'a: 'r>(archetype: &'a Archetype, index: usize) -> Self {
        assert!(index < archetype.len());
        Self { archetype, index }
    }

    unsafe fn read<C: Component>(&self) -> Option<&C> {
        Some(&self.archetype.get_column_as_slice::<C>()?[self.index])
    }
}

struct ArchetypeRowMut<'r> {
    archetype: &'r mut Archetype,
    index: usize,
}

impl<'r> ArchetypeRowMut<'r> {
    fn new<'a: 'r>(archetype: &'a mut Archetype, index: usize) -> Self {
        assert!(index < archetype.len());
        Self { archetype, index }
    }

    unsafe fn read<C: Component>(&self) -> Option<&C> {
        Some(&self.archetype.get_column_as_slice::<C>()?[self.index])
    }

    unsafe fn write<C: Component>(&mut self, value: C) -> Option<&C> {
        let column = self.archetype.get_column_mut::<C>()?;
        let ptr = column.as_ptr_mut::<C>().add(self.index);
        ptr.write(value);
        ptr.as_ref()
    }
}

/// A set of [`Archetype`]'s
pub(crate) struct ArchetypeManager {
    /// Set of [`Archetype`]s, indexed by [`ArchetypeId`].
    archetypes: HashMap<ArchetypeId, Archetype>,

    /// [`ArchetypeId`] counter
    next: AtomicU32,

    /// Index to get which [`ArchetypeId`] correspond to a
    /// specific set of [`ComponentId`]s.
    index: HashMap<Vec<ComponentId>, ArchetypeId>,

    /// Index to get list of [`ArchetypeId`]s that contain
    /// a given [`ComponentId`].
    component_index: HashMap<ComponentId, Vec<ArchetypeId>>,
}

impl Default for ArchetypeManager {
    /// Has default empty Archetype of `[()]`
    fn default() -> Self {
        let unit_comp = ComponentInfo::of::<()>();
        let unit_comp_id = unit_comp.id;
        let unit_arch_id = ArchetypeId(0);
        let unit_arch = Archetype::from_info(unit_arch_id, vec![unit_comp]);
        Self {
            archetypes: HashMap::from([(unit_arch_id, unit_arch)]),
            next: AtomicU32::new(1),
            index: HashMap::from([(vec![unit_comp_id], unit_arch_id)]),
            component_index: HashMap::from([(unit_comp_id, vec![unit_arch_id])]),
        }
    }
}

impl ArchetypeManager {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn get_one<B: Bundle>(&self) -> Option<&Archetype> {
        let ids = B::id();
        self.index.get(&ids).and_then(|arch_id| self.archetypes.get(arch_id))
    }

    pub(crate) fn get_or_insert<B: Bundle>(&mut self) -> &Archetype {
        self.get_or_insert_with_info(B::info())
    }

    pub(crate) fn get_or_insert_with_info(
        &mut self,
        iter: impl IntoIterator<Item = ComponentInfo>,
    ) -> &Archetype {
        let info: Vec<ComponentInfo> = iter.into_iter().collect();
        let ids: Vec<ComponentId> = info.iter().map(|info| info.id).collect();
        if let Some(arch) = self.index.get(&ids) {
            return self.archetypes.get(arch).unwrap();
        }
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        // We start at 1, so if we've wrapped back around to 0 then we could have duplicate id's
        if id == 0 {
            panic!("Too many Archetypes");
        }
        let arch_id = ArchetypeId(id);

        let arch = self.archetypes
            .entry(arch_id)
            .or_insert_with_key(|arch_id| Archetype::from_info(*arch_id, info));
        for comp_id in &ids {
            self.component_index
                .entry(*comp_id)
                .and_modify(|arch_id_list| {
                    arch_id_list.push(arch_id);
                })
                .or_insert(vec![arch_id]);
        }
        self.index.insert(ids, arch_id);
        arch
    }

    pub(crate) fn query_component<C: Component>(&self) -> Option<&[ArchetypeId]> {
        self.component_index
            .get(&TypeId::of::<C>())
            .map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::{Archetype, ArchetypeId, Component, ComponentInfo};
    use crate::entity::Entity;

    #[test]
    fn archetype() {
        static mut LENGTH_DROP_COUNT: u32 = 0;
        #[derive(Clone, PartialEq, PartialOrd, Debug)]
        struct Length(f32);
        impl Component for Length {}
        impl Drop for Length {
            fn drop(&mut self) {
                unsafe {
                    LENGTH_DROP_COUNT += 1;
                };
            }
        }

        static mut FLOW_DROP_COUNT: u32 = 0;
        #[derive(Clone, PartialEq, PartialOrd, Debug)]
        struct Flow(f32);
        impl Component for Flow {}
        impl Drop for Flow {
            fn drop(&mut self) {
                unsafe {
                    FLOW_DROP_COUNT += 1;
                };
            }
        }

        let comps = vec![ComponentInfo::of::<Length>(), ComponentInfo::of::<Flow>()];
        let mut arch = Archetype::from_info_with_capacity(ArchetypeId(1), comps, 4);

        unsafe {
            let mut row = arch.insert(Entity::with_id(1));
            row.write(Length(2.0));
            row.write(Flow(3.0));

            assert_eq!(row.read::<Length>().unwrap(), &Length(2.0));
            assert_eq!(row.read::<Flow>().unwrap(), &Flow(3.0));
        }
        drop(arch);
        // should drop the value inside the Arch as well as the temporary used for comparison.
        unsafe { assert_eq!(LENGTH_DROP_COUNT, 2) };
        unsafe { assert_eq!(FLOW_DROP_COUNT, 2) };
    }
}
