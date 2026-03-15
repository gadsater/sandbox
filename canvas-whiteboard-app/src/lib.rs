use wasm_bindgen::prelude::*;

pub mod canvas;
pub mod model;
pub mod render;
pub mod scripting;

use model::vault::Vault;

/// Global application state exposed to JS.
#[wasm_bindgen]
pub struct App {
    vault: Vault,
}

#[wasm_bindgen]
impl App {
    /// Create a new App with an empty vault.
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Self {
        // Hook panics into console.error for easier debugging.
        console_error_panic_hook();
        Self {
            vault: Vault::new(name.to_string()),
        }
    }

    /// Return the vault serialised as JSON (for JS interop).
    pub fn vault_json(&self) -> String {
        serde_json::to_string_pretty(&self.vault).unwrap_or_default()
    }

    /// Create a top-level shelf and return its hash id.
    pub fn create_shelf(&mut self, name: &str) -> String {
        let shelf = model::shelf::Shelf::new(name.to_string());
        let id = shelf.id.clone();
        self.vault.shelves.push(shelf);
        id
    }

    /// Create a block inside a shelf (identified by hash id) and return its hash id.
    pub fn create_block(&mut self, shelf_id: &str, x: f64, y: f64) -> Option<String> {
        let shelf = self.vault.find_shelf_mut(shelf_id)?;
        let block = model::block::Block::new(x, y);
        let id = block.id.clone();
        shelf.blocks.push(block);
        Some(id)
    }

    /// Set the text content of a block.
    pub fn set_block_text(
        &mut self,
        shelf_id: &str,
        block_id: &str,
        text: &str,
    ) -> bool {
        if let Some(shelf) = self.vault.find_shelf_mut(shelf_id) {
            if let Some(block) = shelf.blocks.iter_mut().find(|b| b.id == block_id) {
                block.set_text(text);
                return true;
            }
        }
        false
    }

    /// Render markdown content of a block to HTML.
    pub fn render_block_markdown(&self, shelf_id: &str, block_id: &str) -> Option<String> {
        let shelf = self.vault.find_shelf(shelf_id)?;
        let block = shelf.blocks.iter().find(|b| b.id == block_id)?;
        Some(render::markdown::render_markdown(&block.content_text()))
    }

    /// Evaluate a script in the embedded scripting language and return the result as a string.
    pub fn eval_script(&self, source: &str) -> String {
        match scripting::eval(source) {
            Ok(val) => format!("{val}"),
            Err(e) => format!("error: {e}"),
        }
    }
}

/// Minimal panic hook – pipes Rust panics to `console.error`.
fn console_error_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    {
        use std::sync::Once;
        static SET_HOOK: Once = Once::new();
        SET_HOOK.call_once(|| {
            std::panic::set_hook(Box::new(|info| {
                web_sys::console::error_1(&format!("{info}").into());
            }));
        });
    }
}
