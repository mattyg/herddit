import test from 'ava';
import assert from 'node:assert';

import { runScenario, pause } from '@holochain/tryorama';
import { Record } from '@holochain/client';
import { decode } from '@msgpack/msgpack';


test('create post', async t => {
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
      title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
      content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
    };

    // Alice creates a post
    const record: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record);

  },
  true,
  { timeout: 25000 });
});

test('create and read post', async t => {
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
  title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};

    // Alice creates a post
    const record: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record);
    
    // Wait for the created entry to be propagated to the other node.
    await pause(2000);

    // Bob gets the created post
    const createReadOutput: Record = await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(createInput, decode((createReadOutput.entry as any).Present.entry) as any);
  },
  true,
  { timeout: 25000 });
});

test('create and update post', async t => {
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
  title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};

    // Alice creates a post
    const record: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the post
    let contentUpdate: any = {
  title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};
    let updateInput = { 
      original_post_hash: originalActionHash,
      previous_post_hash: originalActionHash,
      updated_post: contentUpdate,
    };

    let updatedRecord: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "update_post",
      payload: updateInput,
    });
    t.assert(updatedRecord);


    // Wait for the updated entry to be propagated to the other node.
    await pause(2000);
        
    // Bob gets the updated post
    const readUpdatedOutput0: Record = await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);


    // Alice updates the post again
    contentUpdate = {
  title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
  content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
};
    updateInput = { 
      original_post_hash: originalActionHash,
      previous_post_hash: updatedRecord.signed_action.hashed.hash,
      updated_post: contentUpdate,
    };

    updatedRecord = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "update_post",
      payload: updateInput,
    });
    t.assert(updatedRecord);


    // Wait for the updated entry to be propagated to the other node.
    await pause(2000);
        
    // Bob gets the updated post
    const readUpdatedOutput1: Record = await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);

  },
  true,
  { timeout: 25000 });
});

test('create and delete post', async t => {
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
      title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
      content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
    };

    // Alice creates a post
    const record: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record);
        
    // Alice deletes the post
    const deleteActionHash = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "delete_post",
      payload: record.signed_action.hashed.hash,
    });
    t.assert(deleteActionHash);


    // Wait for the entry deletion to be propagated to the other node.
    await pause(2000);
        
    // Bob tries to get the deleted post
    const deletedRecord: Record = await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(deletedRecord.signed_action.hashed.hash, deleteActionHash);

  },
  true,
  { timeout: 25000 });
});


test('get_all_posts_sorted_by_votes sorts by number of votes desc, then timestamp asc', async t => {
  await runScenario(async scenario => {

    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/' + "../workdir/herddit.happ";

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob, jane, john] = await scenario.addPlayersWithApps([appSource, appSource, appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const createInput = {
      title: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
      content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.',
    };

    // John creates a post
    const record3: Record = await john.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record3);
    
    // Alice creates a post
    const record: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record);

    // John creates a post
    const record2: Record = await john.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record2);

    // Bob creates a post
    const record4: Record = await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.assert(record4);

    // Alice, bob, jane, john upvote alice's post
    await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "upvote_post",
      payload: record.signed_action.hashed.hash,
    });
    await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "upvote_post",
      payload: record.signed_action.hashed.hash,
    });
    await jane.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "upvote_post",
      payload: record.signed_action.hashed.hash,
    });
    await john.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "upvote_post",
      payload: record.signed_action.hashed.hash,
    });

    // Alice, bob upvote bob's post
    await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "upvote_post",
      payload: record4.signed_action.hashed.hash,
    });
    await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "upvote_post",
      payload: record4.signed_action.hashed.hash,
    });

    await pause(2000);


    
    // All agents see the same post ordering
    const alice_posts_sorted = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_all_posts_sorted_by_votes",
    });
    t.deepEqual(alice_posts_sorted, [record.signed_action.hashed.hash, record3.signed_action.hashed.hash, record2.signed_action.hashed.hash])
    
    const bob_posts_sorted = await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_all_posts_sorted_by_votes",
    });
    t.deepEqual(bob_posts_sorted, [record.signed_action.hashed.hash, record3.signed_action.hashed.hash, record2.signed_action.hashed.hash])

    const jane_posts_sorted = await jane.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_all_posts_sorted_by_votes",
    });
    t.deepEqual(jane_posts_sorted, [record.signed_action.hashed.hash, record3.signed_action.hashed.hash, record2.signed_action.hashed.hash])

    const john_posts_sorted = await john.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_all_posts_sorted_by_votes",
    });
    t.deepEqual(john_posts_sorted, [record.signed_action.hashed.hash, record3.signed_action.hashed.hash, record2.signed_action.hashed.hash])

    // Bob removes his vote to alice's post
    await bob.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "rmvote_post",
      payload: record.signed_action.hashed.hash,
    });

    await pause(2000);

    const metadata: Record = await alice.namedCells.get('herd').callZome({
      zome_name: "posts",
      fn_name: "get_post_metadata",
      payload: record.signed_action.hashed.hash,
    });

    t.deepEqual(3, (decode((metadata.entry as any).Present.entry) as any).upvotes);
        
  },
  true,
  { timeout: 100000 }
  );
});
