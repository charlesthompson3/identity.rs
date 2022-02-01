// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Endpoint;
use crate::Result;

use libp2p::PeerId;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage
// where
//   T: Serialize + for<'deser> Deserialize<'deser>,
{
  pub endpoint: Endpoint,
  pub data: Vec<u8>,
}

impl RequestMessage
// where
//   T: Serialize + for<'deser> Deserialize<'deser>,
{
  pub fn new(name: impl AsRef<str>, data: Vec<u8>) -> Result<Self> {
    Ok(Self {
      endpoint: Endpoint::new(name)?,
      data,
    })
  }
}

pub type ResponseMessage = Vec<u8>;

pub struct RequestContext<T> {
  pub input: T,
  pub peer: PeerId,
  pub endpoint: Endpoint,
}

impl<T> RequestContext<T> {
  pub fn new(input: T, peer: PeerId, endpoint: Endpoint) -> Self {
    Self { input, peer, endpoint }
  }

  pub fn convert<I>(self, input: I) -> RequestContext<I> {
    RequestContext::new(input, self.peer, self.endpoint)
  }
}
