// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use wasm_bindgen::JsValue;

/// Convenience wrapper for `Result<T, JsValue>`.
///
/// All exported errors must be converted to [`JsValue`] when using wasm_bindgen.
/// See: https://rustwasm.github.io/docs/wasm-bindgen/reference/types/result.html
pub type Result<T> = core::result::Result<T, JsValue>;

/// Convert an error into an idiomatic [js_sys::Error].
pub fn wasm_error<'a, E>(error: E) -> JsValue
where
  E: Into<WasmError<'a>>,
{
  let wasm_err: WasmError = error.into();
  JsValue::from(wasm_err)
}

/// Convenience trait to simplify `result.map_err(wasm_error)` to `result.wasm_result()`
pub(crate) trait WasmResult<T> {
  fn wasm_result(self) -> Result<T>;
}

impl<'a, T, E> WasmResult<T> for core::result::Result<T, E>
where
  E: Into<WasmError<'a>>,
{
  fn wasm_result(self) -> Result<T> {
    self.map_err(wasm_error)
  }
}

/// Convenience struct to convert internal errors to [js_sys::Error]. Uses [std::borrow::Cow]
/// internally to avoid unnecessary clones.
///
/// This is a workaround for orphan rules so we can implement [core::convert::From] on errors from
/// dependencies.
#[derive(Debug, Clone)]
pub struct WasmError<'a> {
  pub name: Cow<'a, str>,
  pub message: Cow<'a, str>,
}

impl<'a> WasmError<'a> {
  pub fn new(name: Cow<'a, str>, message: Cow<'a, str>) -> Self {
    Self { name, message }
  }
}

/// Convert [WasmError] into [js_sys::Error] for idiomatic error handling.
impl From<WasmError<'_>> for js_sys::Error {
  fn from(error: WasmError<'_>) -> Self {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name(&error.name);
    js_error
  }
}

/// Convert [WasmError] into [wasm_bindgen::JsValue].
impl From<WasmError<'_>> for JsValue {
  fn from(error: WasmError<'_>) -> Self {
    JsValue::from(js_sys::Error::from(error))
  }
}

/// Implement WasmError for each type individually rather than a trait due to Rust's orphan rules.
/// Each type must implement `Into<&'static str> + Display`. The `Into<&'static str>` trait can be
/// derived using `strum::IntoStaticStr`.
#[macro_export]
macro_rules! impl_wasm_error_from {
  ( $($t:ty),* ) => {
  $(impl From<$t> for WasmError<'_> {
    fn from(error: $t) -> Self {
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(error.into()),
      }
    }
  })*
  }
}

impl_wasm_error_from!(
  identity::credential::Error,
  identity::did::Error,
  identity::did::DIDError,
  identity::iota::Error
);

impl From<serde_json::Error> for WasmError<'_> {
  fn from(error: serde_json::Error) -> Self {
    Self {
      name: Cow::Borrowed("serde_json::Error"), // the exact error code is embedded in the message
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<identity::iota::BeeMessageError> for WasmError<'_> {
  fn from(error: identity::iota::BeeMessageError) -> Self {
    Self {
      name: Cow::Borrowed("bee_message::Error"),
      message: Cow::Owned(error.to_string()),
    }
  }
}

// Similar to `impl_wasm_error_from`, but uses the types name instead of requiring/calling Into &'static str
#[macro_export]
macro_rules! impl_wasm_error_from_with_struct_name {
  ( $($t:ty),* ) => {
  $(impl From<$t> for WasmError<'_> {
    fn from(error: $t) -> Self {
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(stringify!($t)),
      }
    }
  })*
  }
}
mod wasm_error_from_identity_core {
  use super::*;
  use identity::core::errors::Base58DecodingError;
  use identity::core::errors::Base64DecodingError;
  use identity::core::errors::FatalError;
  use identity::core::errors::JsonDecodingError;
  use identity::core::errors::JsonEncodingError;
  use identity::core::errors::KeyCollectionError;
  use identity::core::errors::KeyCollectionSizeError;
  use identity::core::errors::KeyPairGenerationError;
  use identity::core::errors::KeyParsingError;
  use identity::core::errors::MerkleDigestKeyTagError;
  use identity::core::errors::MerkleKeyTagExtractionError;
  use identity::core::errors::MerkleSignatureKeyTagError;
  use identity::core::errors::MissingSignatureError;
  use identity::core::errors::ProofSizeError;
  use identity::core::errors::SigningError;
  use identity::core::errors::TimeStampParsingError;
  use identity::core::errors::UrlParsingError;
  use identity::core::errors::VerificationError;

  // Simple conversions just using the name of the struct and their display implementation
  impl_wasm_error_from_with_struct_name!(
    TimeStampParsingError,
    UrlParsingError,
    Base64DecodingError,
    Base58DecodingError,
    KeyPairGenerationError,
    KeyParsingError,
    ProofSizeError,
    JsonDecodingError,
    JsonEncodingError,
    MerkleDigestKeyTagError,
    MerkleSignatureKeyTagError,
    MissingSignatureError,
    SigningError,
    FatalError
  );

  // More involved conversions adding enum variant details to the name:
  impl From<KeyCollectionSizeError> for WasmError<'_> {
    fn from(error: KeyCollectionSizeError) -> Self {
      let name = match error {
        KeyCollectionSizeError::Empty => "KeyCollectionSizeError - Empty",
        KeyCollectionSizeError::NotAPowerOfTwo(_) => "KeyCollectionSizeError - NotAPowerOfTwo",
        KeyCollectionSizeError::MaximumExceeded(_) => "KeyCollectionSizeError - MaximumExceeded",
        KeyCollectionSizeError::KeyPairImbalance { .. } => "KeyCollectionSizeError - KeyPairImbalance",
      };

      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(name),
      }
    }
  }

  impl From<KeyCollectionError> for WasmError<'_> {
    fn from(error: KeyCollectionError) -> Self {
      let name = match error {
        KeyCollectionError::GenerationFailed(_) => "keyCollectionError - GenerationFailed",
        KeyCollectionError::InvalidSize(_) => "KeyCollectionError - InvalidSize",
      };
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(name),
      }
    }
  }

  impl From<MerkleKeyTagExtractionError> for WasmError<'_> {
    fn from(error: MerkleKeyTagExtractionError) -> Self {
      let name = match error {
        MerkleKeyTagExtractionError::InvalidMerkleDigestKeyTag(_) => {
          "MerkleKeyTagExtractionError - InvalidMerkleDigestKeyTag"
        }
        MerkleKeyTagExtractionError::InvalidMerkleSignatureKeyTag(_) => {
          "MerkleKeyTagExtractionError - InvalidSignatureKeyTag"
        }
      };
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(name),
      }
    }
  }

  impl From<VerificationError> for WasmError<'_> {
    fn from(error: VerificationError) -> Self {
      let name = match error {
        VerificationError::InvalidProofValue(_) => "VerificationError - InvalidProofValue",
        VerificationError::ProcessingFailed(_) => "VerificationError - ProcessingFailed",
        VerificationError::Revoked(_) => "VerificationError - Revoked",
      };
      Self {
        message: Cow::Owned(error.to_string()),
        name: Cow::Borrowed(name),
      }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    #[test]
    fn into_wasm_error() {
      let wasm_error: WasmError = FatalError::from("fatal test error".to_string()).into();
      // check that the name of the error type gets used in the name field of WasmError after conversion
      assert_eq!("FatalError", wasm_error.name)
    }
  }
}
