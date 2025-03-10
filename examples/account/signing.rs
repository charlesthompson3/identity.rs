// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example account_signing

use std::path::PathBuf;

use identity::account::Account;
use identity::account::AccountStorage;
use identity::account::IdentitySetup;
use identity::account::Result;
use identity::core::json;
use identity::core::FromJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Subject;
use identity::crypto::KeyPair;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifierOptions;
use identity::did::DID;
use identity::iota::ExplorerUrl;
use identity::iota::ResolvedIotaDocument;
use identity::iota_core::IotaDID;

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init();

  // ===========================================================================
  // Create Identity - Similar to create_did example
  // ===========================================================================

  // Stronghold settings
  let stronghold_path: PathBuf = "./example-strong.hodl".into();
  let password: String = "my-password".into();

  // Create a new Account with stronghold storage.
  let mut account: Account = Account::builder()
    .storage(AccountStorage::Stronghold(stronghold_path, Some(password), None))
    .create_identity(IdentitySetup::default())
    .await?;

  // ===========================================================================
  // Signing Example
  // ===========================================================================

  // Add a new Ed25519 Verification Method to the identity
  account
    .update_identity()
    .create_method()
    .fragment("key-1")
    .apply()
    .await?;

  // Create a subject DID for the recipient of a `UniversityDegree` credential.
  let subject_key: KeyPair = KeyPair::new_ed25519()?;
  let subject_did: IotaDID = IotaDID::new(subject_key.public().as_ref())?;

  // Create the actual Verifiable Credential subject.
  let subject: Subject = Subject::from_json_value(json!({
    "id": subject_did.as_str(),
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  }))?;

  // Issue an unsigned Credential...
  let mut credential: Credential = Credential::builder(Default::default())
    .issuer(Url::parse(account.did().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // ...and sign the Credential with the previously created Verification Method
  account
    .sign("key-1", &mut credential, SignatureOptions::default())
    .await?;

  println!("[Example] Local Credential = {:#}", credential);

  // Fetch the DID Document from the Tangle
  //
  // This is an optional step to ensure DID Document consistency.
  let resolved: ResolvedIotaDocument = account.resolve_identity().await?;

  // Retrieve the DID from the newly created identity.
  let iota_did: &IotaDID = account.did();

  // Prints the Identity Resolver Explorer URL.
  // The entire history can be observed on this page by clicking "Loading History".
  let explorer: &ExplorerUrl = ExplorerUrl::mainnet();
  println!(
    "[Example] Explore the DID Document = {}",
    explorer.resolver_url(iota_did)?
  );

  // Ensure the resolved DID Document can verify the credential signature
  let verified: bool = resolved
    .document
    .verify_data(&credential, &VerifierOptions::default())
    .is_ok();

  println!("[Example] Credential Verified = {}", verified);

  Ok(())
}
