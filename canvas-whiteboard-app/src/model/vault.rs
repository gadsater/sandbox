use serde::{Deserialize, Serialize};

use super::hash;
use super::shelf::Shelf;

/// A vault is the top-level container (like a project / repo).
/// It owns a collection of shelves.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    /// Hash-based reference id.
    pub id: String,
    /// Display name of the vault.
    pub name: String,
    /// Top-level shelves.
    pub shelves: Vec<Shelf>,
}

impl Vault {
    pub fn new(name: String) -> Self {
        let id = hash::make_id(&name);
        Self {
            id,
            name,
            shelves: Vec::new(),
        }
    }

    /// Find a shelf anywhere in the tree by its hash id.
    pub fn find_shelf(&self, id: &str) -> Option<&Shelf> {
        for shelf in &self.shelves {
            if let Some(found) = shelf.find_shelf(id) {
                return Some(found);
            }
        }
        None
    }

    /// Find a mutable shelf anywhere in the tree by its hash id.
    pub fn find_shelf_mut(&mut self, id: &str) -> Option<&mut Shelf> {
        for shelf in &mut self.shelves {
            if let Some(found) = shelf.find_shelf_mut(id) {
                return Some(found);
            }
        }
        None
    }
}
