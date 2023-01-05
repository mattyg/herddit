
import test from 'node:test';
import assert from 'node:assert';

import { runScenario, pause } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource } from '@holochain/client';
import { decode } from '@msgpack/msgpack';


test('create title', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {

    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/' + "../workdir/herddit.happ";

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();


    const createInput = {
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};

    // Alice creates a title
    const record: Record = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "create_title",
      payload: createInput,
    });
    assert.ok(record);

  });
});


test('create and read title', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {

    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/' + "../workdir/herddit.happ";

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const createInput: any = {
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};

    // Alice creates a title
    const record: Record = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "create_title",
      payload: createInput,
    });
    assert.ok(record);
    
    // Wait for the created entry to be propagated to the other node.
    await pause(800);

    // Bob gets the created title
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_title",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(createInput, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update title', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {

    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/' + "../workdir/herddit.happ";

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const createInput = {
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};

    // Alice creates a title
    const record: Record = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "create_title",
      payload: createInput,
    });
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the title
    let contentUpdate: any = {
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};
    let updateInput = { 
      original_title_hash: originalActionHash,
      previous_title_hash: originalActionHash,
      updated_title: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "update_title",
      payload: updateInput,
    });
    assert.ok(updatedRecord);


    // Wait for the updated entry to be propagated to the other node.
    await pause(800);
        
    // Bob gets the updated title
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_title",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);


    // Alice updates the title again
    contentUpdate = {
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};
    updateInput = { 
      original_title_hash: originalActionHash,
      previous_title_hash: updatedRecord.signed_action.hashed.hash,
      updated_title: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "update_title",
      payload: updateInput,
    });
    assert.ok(updatedRecord);


    // Wait for the updated entry to be propagated to the other node.
    await pause(800);
        
    // Bob gets the updated title
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_title",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);

  });
});

test('create and delete title', { concurrency: 1 }, async t => {
  await runScenario(async scenario => {

    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/' + "../workdir/herddit.happ";

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const createInput = {
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};

    // Alice creates a title
    const record: Record = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "create_title",
      payload: createInput,
    });
    assert.ok(record);
        
    // Alice deletes the title
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "delete_title",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);


    // Wait for the entry deletion to be propagated to the other node.
    await pause(800);
        
    // Bob tries to get the deleted title
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_title",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(readDeletedOutput, undefined);

  });
});
