// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {AccountBuilder, Client, Network, ExplorerUrl, Config, DIDMessageEncoding, AutoSave } from './../../node/identity_wasm.js';

/**
 * This example shows some configurations that can be used for the account.
 */
async function config() {

    // Set-up for a private Tangle
    // You can use https://github.com/iotaledger/one-click-tangle for a local setup.
    // The `network_name` needs to match the id of the network or a part of it.
    // As an example we are treating the devnet as a private tangle, so we use `dev`.
    // When running the local setup, we can use `tangle` since the id of the one-click
    // private tangle is `private-tangle`, but we can only use 6 characters.
    // Keep in mind, there are easier ways to change to devnet via `Network::Devnet`
    const network_name = "dev";
    let network = Network.try_from_name(network_name)

    // If you deployed an explorer locally this would usually be `http://127.0.0.1:8082`
    const explorer = ExplorerUrl.parse("https://explorer.iota.org/devnet");

    // In a locally running one-click tangle, this would usually be `http://127.0.0.1:14265`
    let private_node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe";

    // Create a `Config`for the network.
    const config = new Config();
    config.setNetwork(network);

    // This URL points to the REST API of the locally running hornet node.
    config.setPrimaryNode(private_node_url);

    // Use DIDMessageEncoding.Json instead to publish plaintext messages to the Tangle for debugging.
    config.setEncoding(DIDMessageEncoding.JsonBrotli);

    const client = Client.fromConfig(config);


    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder({
        // never auto-save. rely on the drop save.
        // use `AutoSave.every()` to save immediately after every action,
        // and `AutoSave.batch(10)` to save after every 10 actions.
        autoSave: AutoSave.never(),
        autopublish: true, // publish to the tangle automatically on every update
        milestone: 4, // save a snapshot every 4 actions
        client: client // set client to the previously defined client.
    });

    try {
        let account = await builder.createIdentity();
        let did = account.did();

        // Prints the Identity Resolver Explorer URL.
        // The entire history can be observed on this page by clicking "Loading History".
        console.log(`[Example] Explore the DID Document = ${explorer.resolverUrl(did.toString())}`);

    } catch (e: any) {
        console.log(`[Example] Error: ${e.message}`);
        console.log(`[Example] Is your Tangle node listening on ${private_node_url}?`);
    }
}

export { config };
