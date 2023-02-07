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

#[hdk_extern]
pub fn get_comment(original_comment_hash: ActionHash) -> ExternResult<Record> { 
    get_latest_record(original_comment_hash)
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
    let comment_hashes: Vec<ActionHash> = links
        .into_iter()
        .map(|link| ActionHash::from(link.target))
        .collect();

    Ok(comment_hashes)
}
