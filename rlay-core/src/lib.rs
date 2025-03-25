use std::sync::{Arc, LazyLock, Mutex};

#[macro_export]
macro_rules! rlay {
    (@{$($val:ident = $exp:expr),* $(,)?}) => {{
        #[allow(clippy::needless_update)]
        {
            $crate::APP_CTX.lock().unwrap().add_element(
                $crate::RlayElement::new($crate::RlayElementConfig { $($val : $exp),*, ..Default::default() })
            );
        }
    }};
    (@{$($val:ident = $exp:expr),* $(,)?} $child:expr) => {{
        #[allow(clippy::needless_update)]
        {
            $crate::APP_CTX.lock().unwrap().open_element(
                $crate::RlayElement::new($crate::RlayElementConfig { $($val : $exp),*, ..Default::default() })
            );
        }
        {
            $child
        }
        {
            $crate::APP_CTX.lock().unwrap().close_element();
        }
        }};
}

#[derive(Debug, Default)]
pub enum Sizing {
    Fixed(i32),
    #[default]
    Grow,
}

#[derive(Debug, Default)]
pub enum Colors {
    Blue,
    Pink,

    #[default]
    Black,
}

#[derive(Debug, Default)]
pub struct RlayElementConfig {
    pub sizing: [Sizing; 2],
    pub background_color: Colors,
}

#[derive(Debug, Default)]
pub struct AppCtx {
    elements: Vec<Arc<Mutex<RlayElement>>>,
}

impl AppCtx {
    fn get_root(&self) -> Option<Arc<Mutex<RlayElement>>> {
        self.elements.first().map(Arc::clone)
    }

    pub fn open_element(&mut self, el: RlayElement) {
        let new_el = Arc::new(Mutex::new(el));
        if let Some(parent) = self.elements.last() {
            if let Ok(mut parent_lock) = parent.lock() {
                parent_lock.add_child(Arc::clone(&new_el));
            }
            new_el.lock().unwrap().add_parent(Arc::clone(parent));
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
            new_el.lock().unwrap().add_parent(Arc::clone(parent));
        }
    }
}

pub static APP_CTX: LazyLock<Arc<Mutex<AppCtx>>> =
    LazyLock::new(|| Arc::new(Mutex::new(AppCtx::default())));

#[derive(Debug)]
pub struct RlayElement {
    config: RlayElementConfig,
    parent: Option<Arc<Mutex<RlayElement>>>,
    children: Vec<Arc<Mutex<RlayElement>>>,
}

impl RlayElement {
    pub fn new(config: RlayElementConfig) -> Self {
        Self {
            config,
            parent: None,
            children: vec![],
        }
    }

    pub fn add_parent(&mut self, parent: Arc<Mutex<RlayElement>>) {
        self.parent = Some(Arc::clone(&parent));
    }

    pub fn add_child(&mut self, child: Arc<Mutex<RlayElement>>) {
        self.children.push(Arc::clone(&child));
    }
}

impl RlayElement {
    pub fn with(self, f: impl FnMut(&mut Self)) -> RlayElement {
        self
    }
}

pub fn get_root() -> Option<Arc<Mutex<RlayElement>>> {
    APP_CTX.lock().ok()?.get_root()
}
