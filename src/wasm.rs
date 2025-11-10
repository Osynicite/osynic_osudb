//! WASM bindings for osynic_osudb
//!
//! This module provides WebAssembly bindings for the osu! database parser,
//! allowing it to be used in JavaScript/TypeScript environments.

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

// Re-export main types for WASM
use crate::entity::collection::collectiondb::CollectionDB as InnerCollectionDB;
use crate::entity::osu::osudb::OsuDB as InnerOsuDB;
use crate::entity::scores::scoresdb::ScoresDB as InnerScoresDB;

// Configure console error panic hook for better debugging
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// WASM wrapper for OsuDB
#[wasm_bindgen]
pub struct OsuDB {
    inner: InnerOsuDB,
}

#[wasm_bindgen]
impl OsuDB {
    /// Parse OsuDB from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<OsuDB, JsValue> {
        let osu_db = InnerOsuDB::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse OsuDB: {}", e)))?;

        Ok(OsuDB { inner: osu_db })
    }

    /// Parse OsuDB from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<OsuDB, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// WASM wrapper for ScoresDB
#[wasm_bindgen]
pub struct ScoresDB {
    inner: InnerScoresDB,
}

#[wasm_bindgen]
impl ScoresDB {
    /// Parse ScoresDB from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<ScoresDB, JsValue> {
        let scores_db = InnerScoresDB::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse ScoresDB: {}", e)))?;

        Ok(ScoresDB { inner: scores_db })
    }

    /// Parse ScoresDB from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<ScoresDB, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// WASM wrapper for CollectionDB
#[wasm_bindgen]
pub struct CollectionDB {
    inner: InnerCollectionDB,
}

#[wasm_bindgen]
impl CollectionDB {
    /// Parse CollectionDB from bytes
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<CollectionDB, JsValue> {
        let collection_db = InnerCollectionDB::from_bytes(bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse CollectionDB: {}", e)))?;

        Ok(CollectionDB {
            inner: collection_db,
        })
    }

    /// Parse CollectionDB from Uint8Array
    #[wasm_bindgen(js_name = fromUint8Array)]
    pub fn from_uint8_array(array: &Uint8Array) -> Result<CollectionDB, JsValue> {
        let bytes = array.to_vec();
        Self::new(&bytes)
    }

    /// Get the parsed data as a JavaScript object
    #[wasm_bindgen(js_name = toObject)]
    pub fn to_object(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
}

/// Utility functions for WASM
/// Get library version constants
#[wasm_bindgen(js_name = getVersionConstants)]
pub fn get_version_constants() -> JsValue {
    let constants = serde_json::json!({
        "name": "@osynicite/osynic-osudb",
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION")
    });
    serde_wasm_bindgen::to_value(&constants).unwrap_or(JsValue::NULL)
}
