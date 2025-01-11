use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
};

#[derive(Debug, Default)]
pub struct SketchStore {
    data: BTreeMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl SketchStore {
    pub fn get<T: Send + Sync + 'static>(&self) -> &T {
        let type_id = TypeId::of::<T>();

        self.data
            .get(&type_id)
            .and_then(|value| value.downcast_ref())
            .unwrap_or_else(|| {
                panic!(
                    "Store error: get() called before set() for type {:?}",
                    std::any::type_name::<T>()
                )
            })
    }

    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> &mut T {
        let type_id = TypeId::of::<T>();
        self.data
            .get_mut(&type_id)
            .and_then(|value| value.downcast_mut())
            .unwrap_or_else(|| {
                panic!(
                    "Store error: get_mut() called before set() for type {:?}",
                    std::any::type_name::<T>()
                )
            })
    }

    pub fn set<T: Send + Sync + 'static>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();
        self.data.insert(type_id, Box::new(value));
    }

    pub fn take<T: Send + Sync + 'static>(&mut self) -> T {
        let type_id = TypeId::of::<T>();
        *self
            .data
            .remove(&type_id)
            .unwrap_or_else(|| {
                panic!(
                    "Store error: take() called before set() for type {:?}",
                    std::any::type_name::<T>()
                )
            })
            .downcast()
            .expect("Unexpected: could not downcast item from sketch store")
    }
}
