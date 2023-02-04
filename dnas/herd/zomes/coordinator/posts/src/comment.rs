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
    let details = get_details(original_comment_hash, GetOptions::latest())?.ok_or(wasm_error!(WasmErrorInner::Guest("Comment with that hash not found".into())))?;

    match details {
        Details::Record(record_details) => {
            match record_details.deletes.len() {
                0 => {
                    match record_details.updates.len() {
                        // No updates, return original record
                        0 => Ok(record_details.record),
        
                        // Updates exist, return latest record
                        _ => {
                            let mut updates = record_details.updates.clone();
                            updates.sort_by_key(|a| a.hashed.content.timestamp());
                            let latest_update = updates.last().ok_or(wasm_error!(WasmErrorInner::Guest("Latest comment update not found".into())))?;
                        
                            let latest_update_record = get(latest_update.clone().hashed.hash, GetOptions::latest())?.ok_or(wasm_error!(WasmErrorInner::Guest("Latest comment update record not found".into())))?;
        
                            Ok(latest_update_record)
                        }
                    }
                }
                _ => {
                    let mut deletes = record_details.deletes.clone();
                    deletes.sort_by_key(|a| a.hashed.content.timestamp());
                    let earliest_delete = deletes.first().ok_or(wasm_error!(WasmErrorInner::Guest("Earliest comment delete record not found".into())))?;
                
                    let earliest_delete_record = get(earliest_delete.clone().hashed.hash, GetOptions::latest())?.ok_or(wasm_error!(WasmErrorInner::Guest("Earliest comment delete record not found".into())))?;

                    Ok(earliest_delete_record)
                }
            }
        }
        _ => Err(wasm_error!(WasmErrorInner::Guest("Expected AcitonHash".into())))
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
    let comment_hashes: Vec<ActionHash> = links
        .into_iter()
        .map(|link| ActionHash::from(link.target))
        .collect();

    Ok(comment_hashes)
}
