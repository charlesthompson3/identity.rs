// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A Verifiable Presentation (VP) represents a bundle of one or more Verifiable Credentials.
//! This example demonstrates building and usage of VPs.
//!
//! cargo run --example create_vp

use identity::core::Duration;
use identity::core::FromJson;
use identity::core::Timestamp;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::credential::PresentationBuilder;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifierOptions;

use identity::iota::CredentialValidationOptions;
use identity::iota::CredentialValidator;
use identity::iota::FailFast;
use identity::iota::PresentationValidationOptions;
use identity::iota::PresentationValidator;
use identity::iota::Receipt;
use identity::iota::ResolvedIotaDocument;
use identity::iota::Resolver;
use identity::iota::SubjectHolderRelationship;
use identity::prelude::*;

mod common;
mod create_did;

/// Returns a Presentation signed using the supplied challenge.
pub async fn create_presentation(challenge: String) -> Result<Presentation> {
  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (doc_iss, key_iss, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (doc_sub, key_sub, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuers private key.
  doc_iss.sign_data(
    &mut credential,
    key_iss.private(),
    doc_iss.default_signing_method()?.id(),
    SignatureOptions::default(),
  )?;

  // Create an unsigned Presentation from the previously issued Verifiable Credential.
  let mut presentation: Presentation = PresentationBuilder::default()
    .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs")?)
    .holder(Url::parse(doc_sub.id().as_ref())?)
    .credential(credential)
    .build()?;

  // We now sign the presentation with the holder's private key and include a challenge and an expiry timestamp 10
  // minutes from now.
  // A unique random challenge generated by the requester per presentation can mitigate replay attacks
  // (along with other properties like `expires` and `domain`). The expiry timestamp enables the
  // verifier to drop the challenge from memory after a specified amount of time has passed.

  doc_sub.sign_data(
    &mut presentation,
    key_sub.private(),
    doc_sub.default_signing_method()?.id(),
    SignatureOptions::new()
      .challenge(challenge)
      .expires(Timestamp::from_unix(Timestamp::now_utc().to_unix() + 600)?),
  )?;

  Ok(presentation)
}

/// Executes high level validation logic.
pub async fn high_level_validation(presentation_json: &str, options: &PresentationValidationOptions) -> Result<()> {
  // Deserialize the presentation:
  let presentation: Presentation = Presentation::from_json(&presentation_json)?;
  // Validate the presentation and all the credentials included in it.
  let resolver: Resolver = Resolver::new().await?;
  resolver
    .verify_presentation(&presentation, options, None, None, FailFast::Yes)
    .await
}

/// Verifies signatures and that the credential subject is the holder. Nothing else gets verified.
pub async fn low_level_validation(
  presentation_json: &str,
  holder_verifier_options: &VerifierOptions,
  issuer_verifier_options: &VerifierOptions,
) -> Result<()> {
  // In this case we do not care about the issuance date and expiry date of credentials in the presentation
  // we just want to confirm that the holder is always the credential subject and that the signatures are correct.
  // To do this we need to use the low level validation API.

  // Deserialize the presentation:
  let presentation: Presentation = Presentation::from_json(&presentation_json)?;
  // First check that the holder is always the subject
  PresentationValidator::check_holder_is_always_subject(&presentation)?;
  // Now we resolve the holder and issuers concurrently
  let resolver: Resolver = Resolver::new().await?;
  let (holder_doc, issuer_docs): (ResolvedIotaDocument, Vec<ResolvedIotaDocument>) = tokio::try_join!(
    resolver.resolve_presentation_holder(&presentation),
    resolver.resolve_presentation_issuers(&presentation)
  )?;
  // Verify the holders signature
  PresentationValidator::verify_presentation_signature(&presentation, &holder_doc, holder_verifier_options)?;
  // Verify the issuer's signatures
  for credential in presentation.verifiable_credential.iter() {
    CredentialValidator::verify_signature(credential, issuer_docs.as_slice(), issuer_verifier_options)?;
  }
  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Issue a Verifiable Presentation with a newly created DID Document
  // signed with a challenge from the requester:
  let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";
  let presentation: Presentation = create_presentation(challenge.to_owned()).await?;

  // Convert the Verifiable Presentation to JSON before "exchanging" with a verifier.
  let presentation_json: String = presentation.to_json()?;

  // The verifier wants the following requirements to be satisfied:
  // - Signature verification must also check for the required challenge and the to mitigate replay attacks
  // - Presentation validation must fail if credentials expiring within the next 10 hours are encountered
  // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
  let presentation_verifier_options: VerifierOptions = VerifierOptions::new()
    .challenge(challenge.to_owned())
    .allow_expired(false);

  // Do not allow credentials that expire within the next 10 hours
  let credential_validation_options: CredentialValidationOptions = CredentialValidationOptions::default()
    .earliest_expiry_date(
      Timestamp::now_utc()
        .checked_add(Duration::hours(10))
        .ok_or(anyhow::anyhow!(
          "10 hours later than UTC::now was evaluated to be later than 9999AD"
        ))?,
    );

  let presentation_validation_options = PresentationValidationOptions::default()
    .presentation_verifier_options(presentation_verifier_options.clone())
    .shared_validation_options(credential_validation_options)
    .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject);

  // Conveniently validate the presentation
  high_level_validation(presentation_json.as_str(), &presentation_validation_options).await?;

  // If the verifier instead wanted certain validations to be skipped they could do so with the low level API:
  low_level_validation(
    presentation_json.as_str(),
    &presentation_verifier_options,
    &VerifierOptions::default(),
  )
  .await?;

  Ok(())
}
