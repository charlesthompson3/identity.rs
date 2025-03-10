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
use identity::iota::FailFast;
use identity::iota::PresentationValidationOptions;
use identity::iota::Receipt;
use identity::iota::Resolver;
use identity::iota::SubjectHolderRelationship;
use identity::prelude::*;

mod common;
mod create_did;

#[tokio::main]
async fn main() -> Result<()> {
  // ===========================================================================
  // Participants: credential issuer, presentation holder, verifier
  // ===========================================================================

  // Create a signed DID Document/KeyPair for the credential issuer (see create_did.rs).
  let (doc_iss, key_iss, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Create a signed DID Document/KeyPair for the credential subject (see create_did.rs).
  let (doc_sub, key_sub, _): (IotaDocument, KeyPair, Receipt) = create_did::run().await?;

  // Note that in this example the credential subject is the same as the holder of the presentation.

  // The verifier asks the holder for a presentation signed with the following challenge:
  let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";
  // The verifier and holder also agree that the signature should have an expiry date
  // 10 minutes from now.

  // A unique random challenge generated by the requester per presentation can mitigate replay attacks
  // (along with other properties like `expires` and `domain`).

  // ===========================================================================
  // Issuer - creates and issues a verifiable credential to the holder.
  // ===========================================================================

  // Create an unsigned Credential with claims about `subject` specified by `issuer`.
  let mut credential: Credential = common::issue_degree(&doc_iss, &doc_sub)?;

  // Sign the Credential with the issuers private key.
  doc_iss.sign_data(
    &mut credential,
    key_iss.private(),
    doc_iss.default_signing_method()?.id(),
    SignatureOptions::default(),
  )?;

  println!("Credential JSON > {:#}", credential);

  // The credential is then serialized to JSON and transmitted to the holder in a secure manner.
  // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
  let credential_json: String = credential.to_json()?;

  // ===========================================================================
  // Holder - creates a verifiable presentation from the issued credential for the verifier to validate.
  // ===========================================================================

  // Deserialize the credential.
  let credential: Credential = Credential::from_json(credential_json.as_str())?;

  // Create an unsigned Presentation from the previously issued Verifiable Credential.
  let mut presentation: Presentation = PresentationBuilder::default()
    .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs")?)
    .holder(Url::parse(doc_sub.id().as_ref())?)
    .credential(credential)
    .build()?;

  // The holder signs the presentation with their private key and includes the requested challenge and an expiry
  // timestamp.
  doc_sub.sign_data(
    &mut presentation,
    key_sub.private(),
    doc_sub.default_signing_method()?.id(),
    SignatureOptions::new()
      .challenge(challenge.to_string())
      .expires(Timestamp::now_utc().checked_add(Duration::minutes(10)).unwrap()),
  )?;

  // Convert the Verifiable Presentation to JSON to send it to the verifier.
  let presentation_json: String = presentation.to_json()?;

  // ===========================================================================
  // Verifier - receives a verifiable presentation from the holder and validates it.
  // ===========================================================================

  // Deserialize the presentation from the holder:
  let presentation: Presentation = Presentation::from_json(&presentation_json)?;

  // The verifier wants the following requirements to be satisfied:
  // - Signature verification (including checking the requested challenge to mitigate replay attacks)
  // - Presentation validation must fail if credentials expiring within the next 10 hours are encountered
  // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
  // - The issuance date must not be in the future.

  let presentation_verifier_options: VerifierOptions = VerifierOptions::new()
    .challenge(challenge.to_owned())
    .allow_expired(false);

  // Do not allow credentials that expire within the next 10 hours.
  let credential_validation_options: CredentialValidationOptions = CredentialValidationOptions::default()
    .earliest_expiry_date(Timestamp::now_utc().checked_add(Duration::hours(10)).unwrap());

  let presentation_validation_options = PresentationValidationOptions::default()
    .presentation_verifier_options(presentation_verifier_options.clone())
    .shared_validation_options(credential_validation_options)
    .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject);

  // Validate the presentation and all the credentials included in it.
  let resolver: Resolver = Resolver::new().await?;
  resolver
    .verify_presentation(
      &presentation,
      &presentation_validation_options,
      FailFast::FirstError,
      None,
      None,
    )
    .await?;

  // Note that we did not declare a latest allowed issuance date for credentials. This is because we only want to check
  // that the credentials do not have an issuance date in the future which is a default check.

  Ok(())
}
