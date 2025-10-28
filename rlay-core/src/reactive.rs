use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::AppCtx;

pub struct StateValue<T: Clone> {
    key: String,
    store: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    phantom: PhantomData<T>,
}

impl<T: Clone + 'static> StateValue<T> {
    pub fn new<F>(key: String, ctx: &AppCtx, default_val: F) -> Self
    where
        F: Fn() -> T,
    {
        let store = ctx.store();
        let key_exists = { store.lock().unwrap().contains_key(&key) };
        if !key_exists {
            let default_val = Box::new((default_val)());
            store.lock().unwrap().insert(key.clone(), default_val);
        }
        Self {
            key,
            store: ctx.store(),
            phantom: PhantomData,
        }
    }

    pub fn get(&self) -> T {
        let store = &self.store;
        let store = store.lock().unwrap();
        store
            .get(&self.key)
            .and_then(|v| v.downcast_ref::<T>())
            .cloned()
            .unwrap()
    }

    pub fn set(&self, val: T) {
        let store = &self.store;
        store
            .lock()
            .unwrap()
            .insert(self.key.clone(), Box::new(val));
    }
}

#[macro_export]
macro_rules! useState {
    ($ctx:ident, $default_val:expr) => {{
        let key = format!("{}:{}", file!(), line!());
        &mut $crate::reactive::StateValue::new(key, &$ctx, || $default_val)
    }};
}

#[macro_export]
macro_rules! useEffect {
    ($ctx:ident, $effect:block, [$($dep:ident),* $(,)?]) => {{
        let mut apply_effect = false;
        $({
            let val = $crate::useState!($ctx, $dep.get());
            if val.get() != $dep.get() {
                apply_effect = true;
                val.set($dep.get());
            }
        })*
        if apply_effect {
            $effect
        }
    }};
}
