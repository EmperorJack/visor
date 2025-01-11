use state::TypeMap;

pub(crate) static ENGINE_STORE: Store = Store::new();

pub struct Store {
    type_map: TypeMap![Send + Sync],
}

impl Store {
    pub(crate) const fn new() -> Self {
        Self {
            type_map: <TypeMap![Send + Sync]>::new(),
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> &T {
        self.type_map.try_get::<T>().unwrap_or_else(|| {
            panic!(
                "Store error: get() called before set() for type {:?}",
                std::any::type_name::<T>()
            )
        })
    }

    pub fn set<T: Send + Sync + 'static>(&self, state: T) -> bool {
        self.type_map.set(state)
    }
}
