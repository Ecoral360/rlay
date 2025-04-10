use std::{
    fmt::write,
    sync::{Arc, LazyLock, Mutex},
};

use crate::{Element, err::RlayError};

#[derive(Debug, Default)]
pub struct AppCtx {
    root: Option<Arc<Mutex<Element>>>,
    elements: Vec<Arc<Mutex<Element>>>,
}

impl AppCtx {
    pub(crate) fn get_root(&self) -> Option<Arc<Mutex<Element>>> {
        self.root.as_ref().map(Arc::clone)
    }

    pub(crate) fn take_root(&mut self) -> Result<Element, RlayError> {
        let el = self.root.take().ok_or(RlayError::NoRoot)?;
        Arc::into_inner(el)
            .ok_or(RlayError::RootBorrowed)?
            .into_inner()
            .map_err(|_| RlayError::RootCorrupted)
    }

    pub fn open_element(&mut self, el: Element) {
        let new_el = Arc::new(Mutex::new(el));
        if self.root.is_none() {
            self.root = Some(Arc::clone(&new_el));
        }

        if let Some(parent) = self.elements.last() {
            if let Ok(mut parent_lock) = parent.lock() {
                parent_lock.add_child(Arc::clone(&new_el));
            }
            new_el.lock().unwrap().add_parent(Arc::downgrade(parent));
        }
        self.elements.push(new_el);
    }

    pub fn close_element(&mut self) {
        // We don't remove the root
        let el = self.elements.pop();

        el.expect("At least an element")
            .lock()
            .expect("Not corrupted")
            .close();
    }
}

static APP_CTX: LazyLock<Arc<Mutex<AppCtx>>> =
    LazyLock::new(|| Arc::new(Mutex::new(AppCtx::default())));

pub fn get_ctx() -> Arc<Mutex<AppCtx>> {
    Arc::clone(&APP_CTX)
}

pub fn get_root() -> Option<Arc<Mutex<Element>>> {
    APP_CTX.lock().ok()?.get_root()
}

pub fn take_root() -> Result<Element, RlayError> {
    APP_CTX.lock().expect("Cannot lock the AppCtx").take_root()
}
