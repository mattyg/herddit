use crate::utils::*;
use hdk::prelude::*;
use posts_integrity::*;
use std::cmp::Ordering;

#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<Record> {
    let post_hash = create_entry(&EntryTypes::Post(post))?;
    let record = get(post_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created Post"))
    ))?;
    let path = Path::from("all_posts");
    create_link(
        path.path_entry_hash()?,
        post_hash.clone(),
        LinkTypes::AllPosts,
        (),
    )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(my_agent_pub_key, post_hash, LinkTypes::MyPosts, ())?;
    Ok(record)
}

#[hdk_extern]
pub fn get_post(original_post_hash: ActionHash) -> ExternResult<Record> {
    get_latest_record(original_post_hash)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMetadata {
    pub record: Record,
    pub upvotes: usize,
    pub downvotes: usize,
}

#[hdk_extern]
pub fn get_post_metadata(original_post_hash: ActionHash) -> ExternResult<PostMetadata> {
    let post = get_post(original_post_hash.clone())?;

    let mut post_vote_links = get_links(original_post_hash, LinkTypes::PostVoteByAgent, None)?;
    post_vote_links.sort_by(|a, b| -> Ordering {
        if a.target == b.target {
            b.timestamp.cmp(&a.timestamp)
        } else {
            a.target.cmp(&b.target)
        }
    });
    post_vote_links.dedup_by_key(|a| a.target.clone());
    let latest_post_vote_tags: Vec<VoteTag> = post_vote_links
        .into_iter()
        .map(|link| VoteTag::try_from(SerializedBytes::from(UnsafeBytes::from(link.tag.0))))
        .filter_map(Result::ok)
        .collect();

    let upvotes = latest_post_vote_tags
        .clone()
        .into_iter()
        .filter(|post_vote_tag| post_vote_tag.value == 1)
        .count();

    let downvotes = latest_post_vote_tags
        .into_iter()
        .filter(|post_vote_tag| post_vote_tag.value == -1)
        .count();

    let post_metadata = PostMetadata {
        record: post,
        upvotes,
        downvotes,
    };

    Ok(post_metadata)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostInput {
    pub original_post_hash: ActionHash,
    pub previous_post_hash: ActionHash,
    pub updated_post: Post,
}

#[hdk_extern]
pub fn update_post(input: UpdatePostInput) -> ExternResult<Record> {
    let updated_post_hash = update_entry(input.original_post_hash.clone(), &input.updated_post)?;
    create_link(
        input.original_post_hash,
        updated_post_hash.clone(),
        LinkTypes::PostUpdates,
        (),
    )?;
    let record = get(updated_post_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly updated Post"))
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_post(original_post_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_post_hash)
}

#[hdk_extern]
pub fn upvote_post(original_post_hash: ActionHash) -> ExternResult<()> {
    let agent_pubkey = agent_info()?.agent_initial_pubkey;
    create_link(
        agent_pubkey.clone(),
        original_post_hash.clone(),
        LinkTypes::MyVotedPosts,
        (),
    )?;
    create_link(
        original_post_hash,
        agent_pubkey,
        LinkTypes::PostVoteByAgent,
        make_vote_link_tag(1)?,
    )?;
    Ok(())
}

#[hdk_extern]
pub fn downvote_post(original_post_hash: ActionHash) -> ExternResult<()> {
    create_link(
        original_post_hash,
        agent_info()?.agent_initial_pubkey,
        LinkTypes::PostVoteByAgent,
        make_vote_link_tag(-1)?,
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_my_vote_on_post(comment_hash: ActionHash) -> ExternResult<Option<VoteTag>> {
    // Get all votes on this comment
    let vote_links = get_links(comment_hash, LinkTypes::PostVoteByAgent, None)?;

    // Filter only my votes, sort by timestamp
    let my_pubkey = AnyLinkableHash::from(agent_info()?.agent_initial_pubkey);
    let mut my_vote_links: Vec<Link> = vote_links
        .into_iter()
        .filter(|link| link.target == my_pubkey)
        .collect();
    my_vote_links.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let maybe_my_vote = my_vote_links.first();

    match maybe_my_vote {
        Some(my_vote) => {
            let my_vote_tag = VoteTag::try_from(SerializedBytes::from(UnsafeBytes::from(
                my_vote.clone().tag.0,
            )))
            .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;

            Ok(Some(my_vote_tag))
        }
        None => Ok(None),
    }
}
