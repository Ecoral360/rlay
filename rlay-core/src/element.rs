use core::f32;
use std::{
    marker::PhantomData,
    ops::RangeBounds,
    sync::{Arc, Mutex, Weak},
};

use derive_more::From;

use crate::{Dimension2D, ElementConfig, ElementData, Vector2D, err::RlayError};

#[derive(Debug)]
pub struct Element {
    data: ElementData,
    parent: Option<Weak<Mutex<Element>>>,
    pub(crate) children: Vec<Arc<Mutex<Element>>>,
}

impl Element {
    pub fn new(data: ElementData) -> Self {
        Self {
            data,
            parent: None,
            children: vec![],
        }
    }

    pub fn new_container(config: ElementConfig) -> Self {
        Self {
            data: ElementData::Container { config },
            parent: None,
            children: vec![],
        }
    }

    pub fn config(&self) -> ElementConfig {
        match &self.data {
            ElementData::Container { config } => *config,
            ElementData::Text { config, data } => todo!(),
            ElementData::Image { config, data } => todo!(),
        }
    }

    pub fn children(&self) -> &Vec<Arc<Mutex<Element>>> {
        &self.children
    }

    pub fn parent(&self) -> Option<Weak<Mutex<Element>>> {
        self.parent.as_ref().map(Weak::clone)
    }

    pub fn add_parent(&mut self, parent: Weak<Mutex<Element>>) {
        self.parent = Some(Weak::clone(&parent));
    }

    pub fn add_child(&mut self, child: Arc<Mutex<Element>>) {
        self.children.push(Arc::clone(&child));
    }

    pub(crate) fn close(&mut self) {}
}
