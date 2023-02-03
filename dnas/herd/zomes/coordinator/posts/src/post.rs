use std::cmp::Ordering;
use hdk::prelude::*;
use posts_integrity::*;
#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<Record> {
    let post_hash = create_entry(&EntryTypes::Post(post.clone()))?;
    let record = get(post_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Post"))
            ),
        )?;
    let path = Path::from("all_posts");
    create_link(path.path_entry_hash()?, post_hash.clone(), LinkTypes::AllPosts, ())?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(my_agent_pub_key, post_hash.clone(), LinkTypes::MyPosts, ())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_post(original_post_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_post_hash.clone(), LinkTypes::PostUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
    let latest_post_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_post_hash.clone(),
    };
    get(latest_post_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PostMetadata {
    pub record: Record,
    pub upvotes: usize,
    pub downvotes: usize,
}
#[hdk_extern]
pub fn get_post_metadata(
    original_post_hash: ActionHash,
) -> ExternResult<Option<PostMetadata>> {
    let maybe_post = get_post(original_post_hash.clone())?;
    if let Some(post) = maybe_post {
        let mut post_vote_links = get_links(
            original_post_hash,
            LinkTypes::PostVoteByAgent,
            None,
        )?;
        post_vote_links
            .sort_by(|a, b| -> Ordering {
                if a.target == b.target {
                    return b.timestamp.cmp(&a.timestamp);
                } else {
                    return a.target.cmp(&b.target);
                }
            });
        post_vote_links.dedup_by_key(|a| a.target.clone());
        let latest_post_vote_tags: Vec<PostVoteTag> = post_vote_links
            .into_iter()
            .map(|link| PostVoteTag::try_from(
                SerializedBytes::from(UnsafeBytes::from(link.tag.0)),
            ))
            .filter_map(Result::ok)
            .collect();
        let upvotes = latest_post_vote_tags
            .clone()
            .into_iter()
            .filter(|post_vote_tag| post_vote_tag.value == 1)
            .collect::<Vec<PostVoteTag>>()
            .len();
        let downvotes = latest_post_vote_tags
            .into_iter()
            .filter(|post_vote_tag| post_vote_tag.value == -1)
            .collect::<Vec<PostVoteTag>>()
            .len();
        let post_metadata = PostMetadata {
            record: post,
            upvotes: upvotes,
            downvotes: downvotes,
        };
        Ok(Some(post_metadata))
    } else {
        Ok(None)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePostInput {
    pub original_post_hash: ActionHash,
    pub previous_post_hash: ActionHash,
    pub updated_post: Post,
}
#[hdk_extern]
pub fn update_post(input: UpdatePostInput) -> ExternResult<Record> {
    let updated_post_hash = update_entry(
        input.previous_post_hash.clone(),
        &input.updated_post,
    )?;
    create_link(
        input.original_post_hash.clone(),
        updated_post_hash.clone(),
        LinkTypes::PostUpdates,
        (),
    )?;
    let record = get(updated_post_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Post"))
            ),
        )?;
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
        LinkTypes::MyUpvotedPosts,
        (),
    )?;
    create_link(
        original_post_hash,
        agent_pubkey,
        LinkTypes::PostVoteByAgent,
        make_post_vote_tag(1)?,
    )?;
    Ok(())
}
#[hdk_extern]
pub fn downvote_post(original_post_hash: ActionHash) -> ExternResult<()> {
    create_link(
        original_post_hash,
        agent_info()?.agent_initial_pubkey,
        LinkTypes::PostVoteByAgent,
        make_post_vote_tag(-1)?,
    )?;
    Ok(())
}
pub fn make_post_vote_tag(value: i8) -> ExternResult<LinkTag> {
    let tag_bytes = SerializedBytes::try_from(PostVoteTag { value })
        .map_err(|_e| {
            wasm_error!(WasmErrorInner::Guest("serializedbytes error".into()))
        })?;
    Ok(LinkTag::new(tag_bytes.bytes().clone()))
}
