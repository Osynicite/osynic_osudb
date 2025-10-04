//! WASM bindings for osynic_osudb
//!
//! This module provides WebAssembly bindings for the osu! database parser,
//! allowing it to be used in JavaScript/TypeScript environments.

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

// Re-export main types for WASM
use crate::entity::collection::collectiondb::CollectionDB;
use crate::entity::osu::osudb::OsuDB;
use crate::entity::scores::field::replay::Replay;
use crate::entity::scores::scoresdb::ScoresDB;

// Configure console error panic hook for better debugging
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// WASM wrapper for OsuDB
#[wasm_bindgen]
pub struct WasmOsuDB {
    inner: OsuDB,
}

#[wasm_bindgen]
impl WasmOsuDB {
    /// Parse OsuDB from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<WasmOsuDB, JsValue> {
        let osu_db = OsuDB::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse OsuDB: {}", e)))?;

        Ok(WasmOsuDB { inner: osu_db })
    }

    /// Parse OsuDB from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<WasmOsuDB, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the version
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u32 {
        self.inner.version
    }

    /// Get the folder count
    #[wasm_bindgen(getter, js_name = folderCount)]
    pub fn folder_count(&self) -> u32 {
        self.inner.folder_count
    }

    /// Get the player name
    #[wasm_bindgen(getter, js_name = playerName)]
    pub fn player_name(&self) -> Option<String> {
        self.inner.player_name.clone()
    }

    /// Get the number of beatmaps
    #[wasm_bindgen(js_name = beatmapCount)]
    pub fn beatmap_count(&self) -> usize {
        self.inner.beatmaps.len()
    }

    /// Get beatmaps as a JavaScript array
    #[wasm_bindgen(js_name = getBeatmaps)]
    pub fn get_beatmaps(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner.beatmaps)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// WASM wrapper for ScoresDB
#[wasm_bindgen]
pub struct WasmScoresDB {
    inner: ScoresDB,
}

#[wasm_bindgen]
impl WasmScoresDB {
    /// Parse ScoresDB from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<WasmScoresDB, JsValue> {
        let scores_db = ScoresDB::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse ScoresDB: {}", e)))?;

        Ok(WasmScoresDB { inner: scores_db })
    }

    /// Parse ScoresDB from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<WasmScoresDB, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the version
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u32 {
        self.inner.version
    }

    /// Get the number of beatmaps
    #[wasm_bindgen(js_name = beatmapCount)]
    pub fn beatmap_count(&self) -> usize {
        self.inner.beatmaps.len()
    }
}

/// WASM wrapper for CollectionDB
#[wasm_bindgen]
pub struct WasmCollectionDB {
    inner: CollectionDB,
}

#[wasm_bindgen]
impl WasmCollectionDB {
    /// Parse CollectionDB from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<WasmCollectionDB, JsValue> {
        let collection_db = CollectionDB::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse CollectionDB: {}", e)))?;

        Ok(WasmCollectionDB {
            inner: collection_db,
        })
    }

    /// Parse CollectionDB from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<WasmCollectionDB, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the version
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> u32 {
        self.inner.version
    }

    /// Get the number of collections
    #[wasm_bindgen(js_name = collectionCount)]
    pub fn collection_count(&self) -> usize {
        self.inner.collections.len()
    }
}

/// WASM wrapper for Replay
#[wasm_bindgen]
pub struct WasmReplay {
    inner: Replay,
}

#[wasm_bindgen]
impl WasmReplay {
    /// Parse Replay from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<WasmReplay, JsValue> {
        let replay = Replay::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse Replay: {}", e)))?;

        Ok(WasmReplay { inner: replay })
    }

    /// Parse Replay from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<WasmReplay, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get the mode
    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> u8 {
        self.inner.mode as u8
    }

    /// Get the score
    #[wasm_bindgen(getter)]
    pub fn score(&self) -> u32 {
        self.inner.score
    }

    /// Get the player name
    #[wasm_bindgen(getter, js_name = playerName)]
    pub fn player_name(&self) -> Option<String> {
        self.inner.player_name.clone()
    }
}

/// Utility functions for WASM
#[wasm_bindgen]
pub struct WasmUtils;

#[wasm_bindgen]
impl WasmUtils {
    /// Get library version constants
    #[wasm_bindgen(js_name = getVersionConstants)]
    pub fn get_version_constants() -> JsValue {
        let constants = serde_json::json!({
            "name": env!("CARGO_PKG_NAME"),
            "version": env!("CARGO_PKG_VERSION"),
            "description": env!("CARGO_PKG_DESCRIPTION"),
            "CHANGE_20140609": crate::CHANGE_20140609,
            "CHANGE_20191106": crate::CHANGE_20191106,
            "CHANGE_20250107": crate::CHANGE_20250107
        });
        serde_wasm_bindgen::to_value(&constants).unwrap()
    }

    /// Check if compression feature is available
    #[wasm_bindgen(js_name = hasCompression)]
    pub fn has_compression() -> bool {
        cfg!(feature = "compression")
    }
}
