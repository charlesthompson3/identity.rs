// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::KeyLocation;
use wasm_bindgen::prelude::*;

use crate::account::types::WasmGeneration;
use crate::did::WasmMethodType;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = KeyLocation, inspectable)]
#[derive(Serialize)]
pub struct WasmKeyLocation(pub(crate) KeyLocation);

#[wasm_bindgen(js_class = KeyLocation)]
impl WasmKeyLocation {
  #[wasm_bindgen(constructor)]
  pub fn new(method: WasmMethodType, fragment: String, generation: WasmGeneration) -> WasmKeyLocation {
    WasmKeyLocation(KeyLocation::new(method.into(), fragment, generation.into()))
  }

  /// Returns the method type of the key location.
  #[wasm_bindgen(getter)]
  pub fn method(&self) -> WasmMethodType {
    self.0.method().into()
  }

  /// Returns the fragment name of the key location.
  #[wasm_bindgen(getter)]
  pub fn fragment(&self) -> String {
    self.0.fragment().clone().into()
  }

  /// Returns the fragment name of the key location.
  #[wasm_bindgen(getter = fragmentName)]
  pub fn fragment_name(&self) -> String {
    self.0.fragment_name().to_string()
  }

  /// Returns the integration generation when this key was created.
  #[wasm_bindgen(getter)]
  pub fn generation(&self) -> WasmGeneration {
    self.0.generation().into()
  }

  /// Serializes a `KeyLocation` as `Uint8Array`.
  #[wasm_bindgen(js_name = asBytes)]
  pub fn as_bytes(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).wasm_result()
  }
}

impl From<WasmKeyLocation> for KeyLocation {
  fn from(wasm_key_location: WasmKeyLocation) -> Self {
    wasm_key_location.0
  }
}

impl From<KeyLocation> for WasmKeyLocation {
  fn from(wasm_key_location: KeyLocation) -> Self {
    WasmKeyLocation(wasm_key_location)
  }
}
