use std::cmp::Ordering;

use hdk::prelude::*;
use posts_integrity::*;
use crate::utils::*;

#[hdk_extern]
pub fn create_comment(comment: Comment) -> ExternResult<Record> {
    let comment_hash = create_entry(&EntryTypes::Comment(comment.clone()))?;
    create_link(
        comment.post_ah.clone(),
        comment_hash.clone(),
        LinkTypes::PostToComments,
        (),
    )?;
    let record = get(comment_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Comment"))
            ),
        )?;
    Ok(record)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentMetadata {
    pub record: Record,
    pub upvotes: usize,
    pub downvotes: usize,
}

#[hdk_extern]
pub fn get_comment_metadata(
    original_action_hash: ActionHash,
) -> ExternResult<CommentMetadata> {    

    // Get all vote links, count upvotes & downvotes
    let mut vote_links = get_links(
        original_action_hash.clone(),
        LinkTypes::CommentVoteByAgent,
        None,
    )?;
    vote_links
        .sort_by(|a, b| -> Ordering {
            match a.target == b.target {
                true => b.timestamp.cmp(&a.timestamp),
                false => a.target.cmp(&b.target)
            }
        });
    vote_links.dedup_by_key(|a| a.target.clone());
    let latest_vote_tags: Vec<VoteTag> = vote_links
        .into_iter()
        .map(|link| VoteTag::try_from(
            SerializedBytes::from(UnsafeBytes::from(link.tag.0)),
        ))
        .filter_map(Result::ok)
        .collect();
    
    let upvotes = latest_vote_tags
        .clone()
        .into_iter()
        .filter(|vote_tag| vote_tag.value == 1)
        .collect::<Vec<VoteTag>>()
        .len();
    
    let downvotes = latest_vote_tags
        .into_iter()
        .filter(|vote_tag| vote_tag.value == -1)
        .collect::<Vec<VoteTag>>()
        .len();

    // get actual comment
    let maybe_record = get(original_action_hash.clone(), GetOptions::default())?;
    
    match maybe_record {
        Some(record) => {
            let comment_metadata = CommentMetadata {
                record: record,
                upvotes: upvotes,
                downvotes: downvotes,
            };
            
            Ok(comment_metadata)
        },
        None => Err(wasm_error!(WasmErrorInner::Guest("Failed to get record".into())))
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCommentInput {
    pub original_comment_hash: ActionHash,
    pub previous_comment_hash: ActionHash,
    pub updated_comment: Comment,
}

#[hdk_extern]
pub fn update_comment(input: UpdateCommentInput) -> ExternResult<Record> {
    let updated_comment_hash = update_entry(
        input.original_comment_hash.clone(),
        &input.updated_comment,
    )?;

    let record = get(updated_comment_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Comment"))
            ),
        )?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_comment(original_comment_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_comment_hash)
}

#[hdk_extern]
pub fn get_comments_for_post(post_hash: ActionHash) -> ExternResult<Vec<ActionHash>> {
    let links = get_links(post_hash, LinkTypes::PostToComments, None)?;
   
    // Get all posts metadata and sort by votes, descending
    let mut comments_metadata: Vec<CommentMetadata> = links
        .into_iter()
        .filter_map(|link| get_comment_metadata(ActionHash::from(link.target)).ok())
        .collect();
    comments_metadata.sort_by(|a, b| -> Ordering {
        let a_value = (a.upvotes - a.downvotes) as isize;
        let b_value = (b.upvotes - b.downvotes) as isize;
        
        if a_value == b_value {
            // If values equal, order by record timestamp, desc
            return b.record.action().timestamp().cmp(&a.record.action().timestamp());
        } else {
            // Order by highest value, desc
            return b_value.cmp(&a_value);
        }
    });

    // Return hashes
    let hashes: Vec<ActionHash> = comments_metadata
        .into_iter()
        .map(|r| r.record.action_address().clone())
        .collect();

    Ok(hashes)
}


#[hdk_extern]
pub fn upvote_comment(original_post_hash: ActionHash) -> ExternResult<()> {
    let agent_pubkey = agent_info()?.agent_initial_pubkey;
    create_link(
        agent_pubkey.clone(),
        original_post_hash.clone(),
        LinkTypes::MyVotedComments,
        (),
    )?;
    create_link(
        original_post_hash,
        agent_pubkey,
        LinkTypes::CommentVoteByAgent,
        make_vote_link_tag(1)?,
    )?;
    Ok(())
}

#[hdk_extern]
pub fn downvote_comment(original_post_hash: ActionHash) -> ExternResult<()> {
    create_link(
        agent_info()?.agent_initial_pubkey.clone(),
        original_post_hash.clone(),
        LinkTypes::MyVotedComments,
        (),
    )?;
    create_link(
        original_post_hash,
        agent_info()?.agent_initial_pubkey,
        LinkTypes::CommentVoteByAgent,
        make_vote_link_tag(-1)?,
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_my_vote_on_comment(comment_hash: ActionHash) -> ExternResult<Option<VoteTag>> {
    // Get all votes on this comment
    let vote_links = get_links(
        comment_hash.clone(),
        LinkTypes::CommentVoteByAgent,
        None,
    )?;

    // Filter only my votes, sort by timestamp
    let my_pubkey = agent_info()?.agent_initial_pubkey;
    let mut my_vote_links: Vec<Link> = vote_links.into_iter()
        .filter(|link| link.target == AnyLinkableHash::from(my_pubkey.clone()))
        .collect();
    my_vote_links.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let maybe_my_vote = my_vote_links.first();

    match maybe_my_vote {
        Some(my_vote) => {
            let my_vote_tag = VoteTag::try_from(
                SerializedBytes::from(UnsafeBytes::from(my_vote.clone().tag.0)),
            ).map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
        
            Ok(Some(my_vote_tag))
        }
        None => Ok(None)
    }
}