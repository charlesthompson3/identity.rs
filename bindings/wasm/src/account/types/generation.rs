// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::Generation;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Generation)]
#[derive(Deserialize, Serialize)]
pub struct WasmGeneration(pub(crate) Generation);

#[wasm_bindgen(js_class = Generation)]
impl WasmGeneration {
  /// Creates a new `WasmGeneration`.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    WasmGeneration(Generation::new())
  }

  /// Creates a new `WasmGeneration` from a 32-bit integer.
  #[wasm_bindgen(js_name = fromUnsignedInteger)]
  pub fn from_u32(value: u32) -> Self {
    WasmGeneration(Generation::from_u32(value))
  }

  /// Returns the `WasmGeneration` as a 32-bit integer.
  #[wasm_bindgen(js_name = toUnsignedInteger)]
  pub fn to_u32(self) -> u32 {
    self.0.to_u32()
  }

  /// Increments the `WasmGeneration`.
  ///
  /// # Errors
  ///
  /// Fails in case of overflows.
  #[wasm_bindgen(js_name = tryIncrement)]
  pub fn try_increment(self) -> Result<WasmGeneration> {
    self.0.try_increment().map(|x| x.into()).wasm_result()
  }

  /// Decrements the `WasmGeneration`.
  ///
  /// # Errors
  ///
  /// Fails in case of underflow.
  #[wasm_bindgen(js_name = tryDecrement)]
  pub fn try_decrement(self) -> Result<WasmGeneration> {
    self.0.try_decrement().map(|x| x.into()).wasm_result()
  }

  /// Returns a `WasmGeneration` of minimum value.
  #[wasm_bindgen]
  pub fn min() -> WasmGeneration {
    WasmGeneration(Generation::MIN)
  }

  /// Returns a `WasmGeneration` of maximum value.
  #[wasm_bindgen]
  pub fn max() -> WasmGeneration {
    WasmGeneration(Generation::MAX)
  }

  /// Serializes a `Generation` as `Uint8Array`.
  #[wasm_bindgen(js_name = asBytes)]
  pub fn as_bytes(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).wasm_result()
  }

  /// Deserializes a `Uint8Array` as `Generation`.
  #[wasm_bindgen(js_name = fromBytes)]
  pub fn from_bytes(bytes: Vec<u8>) -> Result<WasmGeneration> {
    bincode::deserialize(&bytes).wasm_result()
  }
}

impl From<Generation> for WasmGeneration {
  fn from(generation: Generation) -> Self {
    Self(generation)
  }
}

impl From<WasmGeneration> for Generation {
  fn from(wasm_generation: WasmGeneration) -> Self {
    wasm_generation.0
  }
}

impl Default for WasmGeneration {
  fn default() -> Self {
    Self::new()
  }
}
