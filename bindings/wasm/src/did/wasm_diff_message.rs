// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity::core::ToJson;
use identity::iota_core::DiffMessage;
use identity::iota_core::MessageId;
use wasm_bindgen::prelude::*;

use crate::did::WasmDID;
use crate::did::WasmDocument;
use crate::error::Result;
use crate::error::WasmResult;

/// Defines the difference between two DID `Document`s' JSON representations.
#[wasm_bindgen(js_name = DiffMessage, inspectable)]
pub struct WasmDiffMessage(pub(crate) DiffMessage);

#[wasm_bindgen(js_class = DiffMessage)]
impl WasmDiffMessage {
  /// Returns the DID of the associated DID Document.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmDID {
    WasmDID::from(self.0.id().clone())
  }

  /// Returns the DID of the associated DID Document.
  #[wasm_bindgen(getter = did)]
  pub fn did(&self) -> WasmDID {
    self.id()
  }

  /// Returns the raw contents of the DID Document diff as a JSON string.
  ///
  /// NOTE: clones the data.
  #[wasm_bindgen(getter = diff)]
  pub fn diff(&self) -> Result<String> {
    self.0.diff().to_json().wasm_result()
  }

  /// Returns the message_id of the DID Document diff.
  #[wasm_bindgen(getter = messageId)]
  pub fn message_id(&self) -> String {
    self.0.message_id().to_string()
  }

  /// Sets the message_id of the DID Document diff.
  #[wasm_bindgen(setter = messageId)]
  pub fn set_message_id(&mut self, message_id: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(message_id)
      .map_err(identity::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.set_message_id(message_id);
    Ok(())
  }

  /// Returns the Tangle message id of the previous DID Document diff.
  #[wasm_bindgen(getter = previousMessageId)]
  pub fn previous_message_id(&self) -> String {
    self.0.previous_message_id().to_string()
  }

  /// Sets the Tangle message id of the previous DID Document diff.
  #[wasm_bindgen(setter = previousMessageId)]
  pub fn set_previous_message_id(&mut self, message_id: &str) -> Result<()> {
    let previous_message_id: MessageId = MessageId::from_str(message_id)
      .map_err(identity::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.set_previous_message_id(previous_message_id);
    Ok(())
  }

  /// Returns the `proof` object.
  #[wasm_bindgen(getter)]
  pub fn proof(&self) -> Result<JsValue> {
    match self.0.proof() {
      Some(proof) => JsValue::from_serde(proof).wasm_result(),
      None => Ok(JsValue::NULL),
    }
  }

  /// Returns a new DID Document which is the result of merging `self`
  /// with the given Document.
  pub fn merge(&self, document: &WasmDocument) -> Result<WasmDocument> {
    self.0.merge(&document.0).map(WasmDocument).wasm_result()
  }

  /// Serializes a `DiffMessage` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `DiffMessage` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDiffMessage> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmDiffMessage, DiffMessage);

impl From<DiffMessage> for WasmDiffMessage {
  fn from(document_diff: DiffMessage) -> Self {
    Self(document_diff)
  }
}
