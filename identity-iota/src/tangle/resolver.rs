// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use identity_core::common::Url;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use serde::Serialize;

use crate::chain::ChainHistory;
use crate::chain::DocumentHistory;
use crate::credential::CredentialValidationOptions;
use crate::credential::CredentialValidator;
use crate::credential::FailFast;
use crate::credential::PresentationValidationOptions;
use crate::credential::PresentationValidator;
use crate::did::IotaDID;
use crate::diff::DiffMessage;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::ClientBuilder;
use crate::tangle::NetworkName;
use crate::tangle::TangleResolve;

/// A `Resolver` supports resolving DID Documents across different Tangle networks using
/// multiple [`Clients`][Client].
///
/// Also provides convenience functions for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
#[derive(Debug)]
pub struct Resolver<T = Arc<Client>>
where
  T: Clone + AsRef<Client> + From<Client>,
{
  client_map: HashMap<NetworkName, T>,
}

impl<T> Resolver<T>
where
  T: Clone + AsRef<Client> + From<Client>,
{
  /// Constructs a new [`Resolver`] with a default [`Client`] for
  /// the [`Mainnet`](crate::tangle::Network::Mainnet).
  ///
  /// See also [`Resolver::builder`].
  pub async fn new() -> Result<Self> {
    let client: Client = Client::new().await?;

    let mut client_map: HashMap<NetworkName, T> = HashMap::new();
    client_map.insert(client.network.name(), T::from(client));
    Ok(Self { client_map })
  }

  /// Returns a new [`ResolverBuilder`] with no configured [`Clients`](Client).
  pub fn builder() -> ResolverBuilder {
    ResolverBuilder::new()
  }

  /// Returns the [`Client`] corresponding to the given [`NetworkName`] if one exists.
  pub fn get_client(&self, network_name: &NetworkName) -> Option<&T> {
    self.client_map.get(network_name)
  }

  /// Returns the [`Client`] corresponding to the [`NetworkName`] on the given ['IotaDID'].
  fn get_client_for_did(&self, did: &IotaDID) -> Result<&T> {
    self.get_client(&did.network()?.name()).ok_or_else(|| {
      Error::DIDNotFound(format!(
        "DID network '{}' does not match any resolver client network",
        did.network_str(),
      ))
    })
  }

  /// Fetches the [`IotaDocument`] of the given [`IotaDID`].
  pub async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    let client: &Client = self.get_client_for_did(did)?.as_ref();
    client.read_document(did).await
  }

  /// Fetches the [`DocumentHistory`] of the given [`IotaDID`].
  pub async fn resolve_history(&self, did: &IotaDID) -> Result<DocumentHistory> {
    let client: &Client = self.get_client_for_did(did)?.as_ref();
    client.resolve_history(did).await
  }

  /// Fetches the [`ChainHistory`] of a diff chain starting from an [`IotaDocument`] on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  pub async fn resolve_diff_history(&self, document: &ResolvedIotaDocument) -> Result<ChainHistory<DiffMessage>> {
    let client: &Client = self.get_client_for_did(document.document.id())?.as_ref();
    client.resolve_diff_history(document).await
  }

  /// Fetches the DID Document of the issuer on a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_credential_issuer<U: Serialize>(
    &self,
    credential: &Credential<U>,
  ) -> Result<ResolvedIotaDocument> {
    let issuer: IotaDID = IotaDID::parse(credential.issuer.url().as_str())?;
    self.resolve(&issuer).await
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_presentation_issuers<U, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Vec<ResolvedIotaDocument>> {
    // Extract unique issuers.
    let issuers: HashSet<IotaDID> = presentation
      .verifiable_credential
      .iter()
      .map(|credential| IotaDID::parse(credential.issuer.url().as_str()))
      .collect::<Result<_>>()?;

    // Resolve issuers concurrently.
    futures::future::try_join_all(issuers.iter().map(|issuer| self.resolve(issuer)).collect::<Vec<_>>()).await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, is not a valid [`IotaDID`], or DID resolution fails.
  pub async fn resolve_presentation_holder<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<ResolvedIotaDocument> {
    let holder_url: &Url = presentation.holder.as_ref().ok_or(Error::IsolatedValidationError(
      crate::credential::ValidationError::MissingPresentationHolder,
    ))?;
    let holder: IotaDID = IotaDID::parse(holder_url.as_str())?;
    self.resolve(&holder).await
  }

  /// Verifies a [`Credential`].
  ///
  /// This method resolves the issuer's DID Document and validates the following properties in accordance with
  /// `options`:
  /// - The issuer's signature
  /// - The expiration date
  /// - The issuance date
  /// - The credential's semantic structure.
  ///
  /// If you already have an up to date version of the issuer's resolved DID Document you may want to use
  /// [`CredentialValidator::validate`](CredentialValidator::validate()) in order to avoid an unnecessary resolution.
  ///
  /// # Warning
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated.
  ///  Examples of properties **not** validated by this method includes: credentialStatus, types, credentialSchema,
  /// refreshService **and more**.
  ///
  /// # Errors
  /// If the issuer's DID Document cannot be resolved an error will be returned immediately. Otherwise
  /// an attempt will be made to validate the credential. If the `fail_fast` parameter is "Yes" an error will be
  /// returned upon the first encountered validation failure, otherwise all validation errors will be accumulated in
  /// the returned error.
  pub async fn verify_credential<U: Serialize>(
    &self,
    credential: &Credential<U>,
    options: &CredentialValidationOptions,
    fail_fast: FailFast,
  ) -> Result<()> {
    let issuer = self.resolve_credential_issuer(credential).await?;
    CredentialValidator::validate(credential, options, &issuer, fail_fast).map_err(Into::into)
  }

  /// Verifies a [`Presentation`].
  ///
  /// This method validates the following properties in accordance with `options`
  /// - The holder's signature,
  /// - The relationship between the holder and the credential subjects,
  /// - The semantic structure of the presentation,
  /// - Some properties of the credentials (see [`CredentialValidator::validate` for more
  ///   information](CredentialValidator::validate())).
  ///  
  /// # Warning
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated.
  ///  Examples of properties **not** validated by this method includes: credentialStatus, types, credentialSchema,
  /// refreshService **and more**.
  ///
  /// # Resolution
  /// If `holder` and/or `issuers` is None then this/these DID Document(s) will be resolved. If you already have up to
  /// date versions of all of these DID Documents you may want to instead use
  /// [`PresentationValidator::validate`](PresentationValidator::validate()).
  ///
  /// # Errors
  /// If the `holder` and/or `issuers` DID Documents need to be resolved, but this operation fails then an error will
  /// immediately be returned. Otherwise an attempt will be made to validate the presentation. If the `fail_fast`
  /// parameter is `Yes` an error will be returned upon the first encountered validation failure, otherwise all
  /// validation errors will be accumulated in the returned error.
  pub async fn verify_presentation<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    holder: Option<&ResolvedIotaDocument>,
    issuers: Option<&[ResolvedIotaDocument]>,
    fail_fast: FailFast,
  ) -> Result<()> {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, options, holder, issuers, fail_fast)
      }
      (Some(holder), None) => {
        let issuers = self.resolve_presentation_issuers(presentation).await?;
        PresentationValidator::validate(presentation, options, holder, &issuers, fail_fast)
      }
      (None, Some(issuers)) => {
        let holder = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, options, &holder, issuers, fail_fast)
      }
      (None, None) => {
        let (holder, issuers) = futures::future::try_join(
          self.resolve_presentation_holder(presentation),
          self.resolve_presentation_issuers(presentation),
        )
        .await?;
        PresentationValidator::validate(presentation, options, &holder, &issuers, fail_fast)
      }
    }
    .map_err(Into::into)
  }
}

/// Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].
#[derive(Default)]
pub struct ResolverBuilder<T = Arc<Client>>
where
  T: Clone + AsRef<Client> + From<Client>,
{
  clients: HashMap<NetworkName, ClientOrBuilder<T>>,
}

#[allow(clippy::large_enum_variant)]
enum ClientOrBuilder<T> {
  Client(T),
  Builder(ClientBuilder),
}

impl<T> ResolverBuilder<T>
where
  T: Clone + AsRef<Client> + From<Client>,
{
  /// Constructs a new [`ResolverBuilder`] with no [`Clients`][Client] configured.
  pub fn new() -> Self {
    Self {
      clients: Default::default(),
    }
  }

  /// Inserts a [`Client`].
  ///
  /// NOTE: replaces any previous [`Client`] or [`ClientBuilder`] with the same [`NetworkName`].
  #[must_use]
  pub fn client(mut self, client: T) -> Self {
    self
      .clients
      .insert(client.as_ref().network.name(), ClientOrBuilder::Client(client));
    self
  }

  /// Inserts a [`ClientBuilder`].
  ///
  /// NOTE: replaces any previous [`Client`] or [`ClientBuilder`] with the same [`NetworkName`].
  pub fn client_builder(mut self, builder: ClientBuilder) -> Self {
    self
      .clients
      .insert(builder.network.name(), ClientOrBuilder::Builder(builder));
    self
  }

  /// Constructs a new [`Resolver`] based on the builder configuration.
  pub async fn build(self) -> Result<Resolver<T>> {
    let mut client_map: HashMap<NetworkName, T> = HashMap::new();
    for (network_name, client_or_builder) in self.clients {
      let client: T = match client_or_builder {
        ClientOrBuilder::Client(client) => client,
        ClientOrBuilder::Builder(builder) => T::from(builder.build().await?),
      };
      client_map.insert(network_name, client);
    }

    Ok(Resolver { client_map })
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for Resolver {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.resolve(did).await
  }
}
