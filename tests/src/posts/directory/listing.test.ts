import test from "ava";
import assert from "node:assert";
import { runScenario, pause } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeDnaHash,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

test("create listing", async (t) => {
    await runScenario(
      async (scenario) => {
        // Construct proper paths for your app.
        // This assumes app bundle created by the `hc app pack` command.
        const testAppPath = process.cwd() + "/" + "../workdir/herddit.happ";

        // Set up the app to be installed
        const appSource = { appBundleSource: { path: testAppPath }, options: {} };

        // Add 2 players with the test app to the Scenario. The returned players
        // can be destructured.
        const [alice, bob] = await scenario.addPlayersWithApps([
          appSource,
          appSource,
        ]);

        // Shortcut peer discovery through gossip and register all agents in every
        // conductor of the scenario.
        await scenario.shareAllAgents();

        const dna = await fakeDnaHash();
        const createInput = {
          title: "ListingName",
          description:
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
          network_seed: '124356',
          dna,
        };

        // Alice creates a listing
        const record: Record = await alice.namedCells.get('directory').callZome({
          zome_name: "directory",
          fn_name: "create_listing",
          payload: createInput,
        });

        t.assert(record);
      },
      true,
      { timeout: 25000 }
    );
});

test("create and read listing", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/" + "../workdir/herddit.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const dna = await fakeDnaHash();
    const createInput: any = {
      title: "Title",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      network_seed: '124356',
      dna,
    };

    // Alice creates a listing
    const hash: ActionHash = await alice.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "create_listing",
      payload: createInput,
    });
    t.assert(hash);

    // Wait for the created entry to be propagated to the other node.
    await pause(800);

    // Bob gets the created listing
    const createReadOutput: Record = await bob.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "get_listing",
      payload: hash,
    });

    assert.deepEqual(
      createInput,
      decode((createReadOutput.entry as any).Present.entry) as any
    );
  },
  true,
  { timeout: 25000 });
});

test("create and update listing", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/" + "../workdir/herddit.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const dna = await fakeDnaHash();
    const createInput = {
      title: "Title",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      network_seed: '124356',
      dna,
    };

    // Alice creates a listing
    const hash: ActionHash = await alice.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "create_listing",
      payload: createInput,
    });
    t.assert(hash);

    // Alice updates the listing
    let contentUpdate: any = {
      title: "NewTitle",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      network_seed: '124356',
      dna,
    };
    let updateInput = {
      original_listing_hash: hash,
      previous_listing_hash: hash,
      updated_listing: contentUpdate,
    };

    let updatedRecord: Record = await alice.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "update_listing",
      payload: updateInput,
    });
    t.assert(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(800);

    // Bob gets the updated listing
    const readUpdatedOutput0: Record = await bob.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "get_listing",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput0.entry as any).Present.entry) as any
    );

    // Alice updates the listing again
    contentUpdate = {
      title: "AnotherNewTitle",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      network_seed: '124356',
      dna,
    };
    updateInput = {
      original_listing_hash: hash,
      previous_listing_hash: updatedRecord.signed_action.hashed.hash,
      updated_listing: contentUpdate,
    };

    updatedRecord = await alice.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "update_listing",
      payload: updateInput,
    });
    t.assert(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(800);

    // Bob gets the updated listing
    const readUpdatedOutput1: Record = await bob.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "get_listing",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput1.entry as any).Present.entry) as any
    );
  },
  true,
  { timeout: 25000 });
});

test("create and delete listing", async (t) => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/" + "../workdir/herddit.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const dna = await fakeDnaHash();
    const createInput = {
      title: "Title",
      description:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.",
      network_seed: '124356',
      dna,
    };

    // Alice creates a listing
    const hash: ActionHash = await alice.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "create_listing",
      payload: createInput,
    });
    t.assert(hash);

    // Alice deletes the listing
    const deleteActionHash = await alice.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "delete_listing",
      payload: hash,
    });
    t.assert(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(800);

    // Bob tries to get the deleted listing
    const readDeletedOutput = await bob.namedCells.get('directory').callZome({
      zome_name: "directory",
      fn_name: "get_listing",
      payload: hash,
    });
    assert.equal(readDeletedOutput, undefined);
  },
  true,
  { timeout: 25000 });
});
