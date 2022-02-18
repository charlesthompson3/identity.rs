// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    ExplorerUrl, AccountBuilder, KeyPair, KeyType, DID, Credential, VerifierOptions, SignatureOptions
} from "../../../wasm/node/identity_wasm.js";
import { Stronghold } from '../../code/stronghold_storage.js'

/**
 * This example demonstrates how to issue and sign Verifiable Credentials using the account.
 */
async function signing() {

    // ===========================================================================
    // Create Identity - Similar to create_did example
    // ===========================================================================

    // Sets the location and password for the Stronghold
    //
    // Stronghold is an encrypted file that manages private keys.
    // It implements best practices for security and is the recommended way of handling private keys.
    let strongholdPath = "./example-strong.hodl";
    let password = "my-password";
    let stronghold = new Stronghold(strongholdPath, password, true);

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder({
        storage: stronghold
    });
    let account = await builder.createIdentity();

    //ToDo: Add Stronghold storage.

    // ===========================================================================
    // Signing Example
    // ===========================================================================

    // Add a new Ed25519 Verification Method to the identity.
    await account.createMethod({
        fragment: "key_1"
    })

    // Create a subject DID for the recipient of a `UniversityDegree` credential.
    let keyPair = new KeyPair(KeyType.Ed25519);
    let subjectDid = new DID(keyPair);

    // Prepare a credential subject indicating the degree earned by Alice.
    let credentialSubject = {
        id: subjectDid.toString(),
        name: "Alice",
        degree: {
            type: "BachelorDegree",
            name: "Bachelor of Science and Arts"
        }
    };

    // Issue an unsigned Credential...
    const unsignedVc = Credential.extend({
        issuer: account.did().toString(),
        type: "UniversityDegreeCredential",
        credentialSubject,
    });

    // ...and sign the Credential with the previously created Verification Method.
    // Note: Different methods are available for different data types,
    // use the Method `createSignedData` to sign arbitrary data.
    let signedVc = await account.createSignedCredential("key_1", unsignedVc, SignatureOptions.default());

    console.log("[Example] Local Credential", signedVc);

    // Fetch the DID Document from the Tangle.
    //
    // This is an optional step to ensure DID Document consistency.
    let resolved = await account.resolveIdentity();

    // Retrieve the DID from the newly created identity.
    let did = account.did().toString();

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));

    // Ensure the resolved DID Document can verify the credential signature.
    let verified = resolved.intoDocument().verifyData(signedVc, VerifierOptions.default());

    console.log("[Example] Credential Verified = ", verified);
}

export { signing }
