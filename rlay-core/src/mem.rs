use std::vec;
use thiserror::Error;

use crate::Element;

#[derive(Error, Debug)]
pub enum MemError {
    #[error("No node at index {{0}}")]
    UndefinedNode(usize),

    #[error("Parent already defined for node {{0}}")]
    ParentAlreadyDefined(usize),
}

pub struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T: PartialEq> Default for ArenaTree<T> {
    fn default() -> Self {
        Self {
            arena: Default::default(),
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn get_next_idx(&mut self, val: &T) -> usize {
        for node in &self.arena {
            if &node.val == val {
                return node.idx;
            }
        }
        self.arena.len()
    }

    fn node_exists(&self, idx: usize) -> bool {
        self.arena.get(idx).is_some()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Node<T>> {
        self.arena.get_mut(idx)
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    pub fn clear(&mut self) {
        self.arena.clear();
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn get_val(&self, idx: usize) -> Option<&T> {
        self.arena.get(idx).map(Node::val)
    }

    pub fn get_children_val(&self, parent_idx: usize) -> Option<Box<[&T]>> {
        Some(
            self.arena
                .get(parent_idx)?
                .children
                .iter()
                .map(|child| self.get_val(*child).expect("Valid child index"))
                .collect(),
        )
    }

    pub fn find(&self, val: &T) -> Option<usize> {
        for node in &self.arena {
            if &node.val == val {
                return Some(node.idx);
            }
        }
        None
    }

    pub fn set(&self, val: T) -> Option<usize> {
        for node in &self.arena {
            if node.val == val {
                return Some(node.idx);
            }
        }
        None
    }

    pub fn insert_node(&mut self, val: T, parent_idx: Option<usize>) -> Result<usize, MemError> {
        // If we provide a parent, make sure it exists
        if let Some(parent_idx) = parent_idx {
            if !self.node_exists(parent_idx) {
                return Err(MemError::UndefinedNode(parent_idx));
            }
        }

        let node_idx = self.get_next_idx(&val);

        self.arena
            .push(Node::new_with_parent(node_idx, val, parent_idx));

        if let Some(parent_idx) = parent_idx {
            self.arena[parent_idx].children.push(node_idx);
        }

        Ok(node_idx)
    }

    pub fn set_parent(&mut self, parent_idx: usize, child_idx: usize) -> Result<(), MemError> {
        if !self.node_exists(parent_idx) {
            return Err(MemError::UndefinedNode(parent_idx));
        }
        if !self.node_exists(child_idx) {
            return Err(MemError::UndefinedNode(child_idx));
        }

        let child = &mut self.arena[child_idx];

        if child.parent.is_some() {
            return Err(MemError::ParentAlreadyDefined(child_idx));
        }

        child.parent = Some(parent_idx);

        let parent = &mut self.arena[parent_idx];
        parent.children.push(child_idx);

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }

    fn new_with_parent(idx: usize, val: T, parent: Option<usize>) -> Self {
        Self {
            idx,
            val,
            parent,
            children: vec![],
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }
}

pub type ElementNode = Node<Element>;
pub type ArenaElement = ArenaTree<Element>;

impl ArenaElement {
    pub fn get_element_with_id(&self, id: &str) -> Option<&Element> {
        for el in self.arena.iter() {
            if el.val.id().is_some_and(|el_id| el_id == id) {
                return Some(&el.val)
            }
        }
        None
    }
}
