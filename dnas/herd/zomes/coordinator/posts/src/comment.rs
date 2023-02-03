use hdk::prelude::*;
use posts_integrity::*;
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
    warn!("this is the input hash {:?}", original_comment_hash);
    
    let links = get_links(
        original_comment_hash.clone(),
        LinkTypes::CommentUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
    let latest_comment_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_comment_hash.clone(),
    };

    let comment_record = get(latest_comment_hash, GetOptions::default())?.ok_or(wasm_error!(WasmErrorInner::Guest("Comment with that hash not found".into())))?;
    
    Ok(comment_record)
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
        input.previous_comment_hash.clone(),
        &input.updated_comment,
    )?;
    create_link(
        input.original_comment_hash.clone(),
        updated_comment_hash.clone(),
        LinkTypes::CommentUpdates,
        (),
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
