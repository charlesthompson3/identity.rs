// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::tangle::MessageId;

use crate::document::ResolvedIotaDocument;
use crate::error::Result;

pub trait TangleRef {
  fn did(&self) -> &IotaDID;

  fn message_id(&self) -> &MessageId;

  fn set_message_id(&mut self, message_id: MessageId);

  fn previous_message_id(&self) -> &MessageId;

  fn set_previous_message_id(&mut self, message_id: MessageId);
}

// TODO: remove TangleResolve with ClientMap refactor?
#[async_trait::async_trait(?Send)]
pub trait TangleResolve {
  /// Resolves a DID on the Tangle
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument>;
}

impl TangleRef for DiffMessage {
  fn did(&self) -> &IotaDID {
    self.id()
  }

  fn message_id(&self) -> &MessageId {
    self.message_id()
  }

  fn set_message_id(&mut self, message_id: MessageId) {
    self.set_message_id(message_id);
  }

  fn previous_message_id(&self) -> &MessageId {
    self.previous_message_id()
  }

  fn set_previous_message_id(&mut self, message_id: MessageId) {
    self.set_previous_message_id(message_id);
  }
}
