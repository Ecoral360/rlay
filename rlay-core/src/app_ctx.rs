use std::{
    fmt::write,
    sync::{Arc, LazyLock, Mutex},
};

use crate::{err::RlayError, RlayElement};

#[derive(Debug, Default)]
pub struct AppCtx {
    elements: Vec<Arc<Mutex<RlayElement>>>,
}

impl AppCtx {
    pub(crate) fn get_root(&self) -> Option<Arc<Mutex<RlayElement>>> {
        self.elements.first().map(Arc::clone)
    }

    pub(crate) fn take_root(&mut self) -> Result<RlayElement, RlayError> {
        let el = self.elements.pop().ok_or(RlayError::NoRoot)?;
        Arc::into_inner(el)
            .ok_or(RlayError::RootBorrowed)?
            .into_inner()
            .map_err(|_| RlayError::RootCorrupted)
    }

    pub fn open_element(&mut self, el: RlayElement) {
        let new_el = Arc::new(Mutex::new(el));
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
        if self.elements.len() > 1 {
            self.elements.pop();
        }
    }

    pub fn add_element(&mut self, el: RlayElement) {
        let new_el = Arc::new(Mutex::new(el));
        if let Some(parent) = self.elements.last() {
            if let Ok(mut parent_lock) = parent.lock() {
                parent_lock.add_child(Arc::clone(&new_el));
            }
            new_el.lock().unwrap().add_parent(Arc::downgrade(parent));
        }
    }
}

static APP_CTX: LazyLock<Arc<Mutex<AppCtx>>> =
    LazyLock::new(|| Arc::new(Mutex::new(AppCtx::default())));

pub fn get_ctx() -> Arc<Mutex<AppCtx>> {
    Arc::clone(&APP_CTX)
}

pub fn get_root() -> Option<Arc<Mutex<RlayElement>>> {
    APP_CTX.lock().ok()?.get_root()
}

pub fn take_root() -> Result<RlayElement, RlayError> {
    APP_CTX.lock().expect("Cannot lock the AppCtx").take_root()
}
