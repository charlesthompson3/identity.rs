// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use identity::account::DetachMethodRelationshipBuilder;
use identity::account::UpdateError::MissingRequiredField;
use identity::account::Account;
use identity::account::IdentityUpdater;
use identity::core::OneOrMany;

use identity::did::MethodRelationship;
use wasm_bindgen::__rt::{RefMut, WasmRefCell};

use crate::account::wasm_account::WasmAccount;
use crate::account::wasm_method_relationship::WasmMethodRelationship;
use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Detaches the given relationship from the given method, if the method exists.
  #[wasm_bindgen(js_name = detachMethodRelationships)]
  pub fn detach_relationships(&mut self, options: &DetachMethodRelationshipOptions) -> Result<Promise> {
    let relationships: Vec<MethodRelationship> = options
      .relationships()
      .into_serde::<OneOrMany<WasmMethodRelationship>>()
      .map(OneOrMany::into_vec)
      .wasm_result()?
      .into_iter()
      .map(Into::into)
      .collect();

    if relationships.is_empty() {
      return Err(wasm_error(MissingRequiredField("relationships is missing or empty")));
    }
    let account: Rc<WasmRefCell<Account>> = Rc::clone(&self.0);
    let fragment: String = options
      .fragment()
      .ok_or(MissingRequiredField("fragment"))
      .wasm_result()?;

    let promise: Promise = future_to_promise(async move {
      let mut account: RefMut<Account> = account.as_ref().borrow_mut();
      let mut updater: IdentityUpdater<'_> = account.update_identity();
      let mut detach_relationship: DetachMethodRelationshipBuilder<'_> =
        updater.detach_method_relationship().fragment(fragment);

      for relationship in relationships {
        detach_relationship = detach_relationship.relationship(relationship);
      }

      detach_relationship
        .apply()
        .await
        .wasm_result()
        .map(|_| JsValue::undefined())
    });
    Ok(promise)
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DetachMethodRelationshipOptions")]
  pub type DetachMethodRelationshipOptions;

  #[wasm_bindgen(getter, method)]
  pub fn fragment(this: &DetachMethodRelationshipOptions) -> Option<String>;

  #[wasm_bindgen(getter, method)]
  pub fn relationships(this: &DetachMethodRelationshipOptions) -> JsValue;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_DETACH_METHOD_RELATIONSHIP_OPTIONS: &'static str = r#"
/**
 * Options for detaching one or more verification relationships from a method on an identity.
 */
export type DetachMethodRelationshipOptions = {
    /**
     * The identifier of the method in the document.
     */
    fragment: string,

    /**
     * The relationships to remove.
     */
    relationships: MethodRelationship | MethodRelationship[]
};
"#;
