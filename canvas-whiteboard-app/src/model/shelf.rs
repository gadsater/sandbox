use serde::{Deserialize, Serialize};

use super::block::Block;
use super::hash;

/// A shelf is a canvas surface that contains blocks.
/// Shelves can be nested (sub-shelves), similar to directories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shelf {
    /// Hash-based reference id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Blocks that live on this canvas.
    pub blocks: Vec<Block>,
    /// Sub-shelves (nested canvases / folders).
    pub children: Vec<Shelf>,
}

impl Shelf {
    pub fn new(name: String) -> Self {
        let id = hash::make_id(&name);
        Self {
            id,
            name,
            blocks: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Add a sub-shelf and return its id.
    pub fn add_child(&mut self, name: String) -> String {
        let child = Shelf::new(name);
        let id = child.id.clone();
        self.children.push(child);
        id
    }

    /// Recursively find a shelf by id (including self).
    pub fn find_shelf(&self, id: &str) -> Option<&Shelf> {
        if self.id == id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_shelf(id) {
                return Some(found);
            }
        }
        None
    }

    /// Recursively find a mutable shelf by id (including self).
    pub fn find_shelf_mut(&mut self, id: &str) -> Option<&mut Shelf> {
        if self.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_shelf_mut(id) {
                return Some(found);
            }
        }
        None
    }
}
