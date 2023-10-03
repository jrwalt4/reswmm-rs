use std::{
    alloc::{alloc, Layout},
    any::{type_name, TypeId},
    collections::HashMap,
    mem::needs_drop,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::entity::{Entity, EntityId};

pub trait Component: Send + Sync + 'static {}

impl Component for () {}

pub type ComponentId = TypeId;

pub(crate) struct ComponentInfo {
    id: ComponentId,
    layout: Layout,
    drop: Option<unsafe fn(*mut u8)>,
    #[cfg(debug_assertions)]
    name: &'static str,
}

impl ComponentInfo {
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

#[repr(transparent)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub(crate) struct ArchetypeId(u32);

pub struct Archetype {
    id: ArchetypeId,
    entities: Vec<EntityId>,
    capacity: usize,
    component_ids: Vec<ComponentId>,
    components: Vec<ComponentColumn>,
}

impl Archetype {
    fn with_capacity(
        id: ArchetypeId,
        components: impl IntoIterator<Item = ComponentInfo>,
        capacity: usize,
    ) -> Self {
        let mut component_info = components.into_iter().collect::<Vec<ComponentInfo>>();
        component_info.sort_by_key(|info| info.id);
        let (component_ids, components) = component_info
            .into_iter()
            .map(|info| {
                (
                    info.id,
                    ComponentColumn::with_capacity(info, capacity).unwrap(),
                )
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

    fn new(id: ArchetypeId, components: impl IntoIterator<Item = ComponentInfo>) -> Self {
        Self::with_capacity(id, components, 0)
    }

    fn component_ids(&self) -> &[ComponentId] {
        &self.component_ids
    }

    fn size(&self) -> usize {
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

    fn insert(&mut self, entity: Entity) -> ArchetypeRowMut<'_> {
        let new_index = self.entities.len();
        self.entities.push(entity.id());
        ArchetypeRowMut::new(self, new_index)
    }
}

pub(crate) struct ArchetypeRow<'a> {
    archetype: &'a Archetype,
    index: usize,
}

impl<'r> ArchetypeRow<'r> {
    fn new<'a: 'r>(archetype: &'a Archetype, index: usize) -> Self {
        assert!(index < archetype.size());
        Self { archetype, index }
    }

    unsafe fn read<C: Component>(&self) -> Option<&C> {
        let data = self.archetype.get_column::<C>()?;
        data.as_ptr::<C>().add(self.index).as_ref()
    }
}

struct ArchetypeRowMut<'r> {
    archetype: &'r mut Archetype,
    index: usize,
}

impl<'r> ArchetypeRowMut<'r> {
    fn new<'a: 'r>(archetype: &'a mut Archetype, index: usize) -> Self {
        assert!(index < archetype.size());
        Self { archetype, index }
    }

    unsafe fn read<C: Component>(&self) -> Option<&C> {
        let data = self.archetype.get_column::<C>()?;
        data.as_ptr::<C>().add(self.index).as_ref()
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
    archetypes: HashMap<ArchetypeId, Archetype>,
    next: AtomicU32,
    index: HashMap<Vec<ComponentId>, ArchetypeId>,
    component_index: HashMap<ComponentId, Vec<ArchetypeId>>,
}

impl Default for ArchetypeManager {
    /// Had default empty Archetype of `[()]`
    fn default() -> Self {
        let unit_comp = ComponentInfo::of::<()>();
        let unit_comp_id = unit_comp.id;
        let unit_arch_id = ArchetypeId(0);
        let unit_arch = Archetype::new(unit_arch_id, vec![unit_comp]);
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

    pub(crate) fn create(
        &mut self,
        components: impl IntoIterator<Item = ComponentInfo>,
    ) -> Option<ArchetypeId> {
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        // We start at 1, so if we've wrapped back around to 0 then we could have duplicate id's
        if id == 0 {
            panic!("Too many Archetypes");
        }
        let id = ArchetypeId(id);
        self.archetypes
            .insert(id, Archetype::new(id, components))
            .map(|arch| {
                self.index.insert(arch.component_ids, arch.id);
                arch.id
            })
    }

    pub(crate) fn query_component<C: Component>(&self) -> Option<&[ArchetypeId]> {
        self.component_index.get(&TypeId::of::<C>()).map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::{Archetype, ArchetypeId, Component, ComponentInfo};
    use crate::entity::Entity;

    #[test]
    fn archetype() {
        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        struct Length(f32);
        impl Component for Length {}

        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        struct Flow(f32);
        impl Component for Flow {}

        let comps = vec![ComponentInfo::of::<Length>(), ComponentInfo::of::<Flow>()];
        let mut arch = Archetype::with_capacity(ArchetypeId(1), comps, 4);

        unsafe {
            let mut row = arch.insert(Entity::with_id(1));
            row.write(Length(2.0));
            row.write(Flow(3.0));

            assert_eq!(row.read::<Length>().copied().unwrap(), Length(2.0));
            assert_eq!(row.read::<Flow>().copied().unwrap(), Flow(3.0));
        }
    }
}
