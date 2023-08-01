use std::cmp::Ordering;

use crate::post::*;
use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
pub fn get_all_posts(_: ()) -> ExternResult<Vec<ActionHash>> {
    let path = Path::from("all_posts");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllPosts, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .filter_map(|link| ActionHash::try_from(link.target).ok())
        .map(|target| GetInput::new(target.into(), GetOptions::default()))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let hashes: Vec<ActionHash> = records
        .into_iter()
        .filter_map(|r| r)
        .map(|r| r.action_address().clone())
        .collect();
    Ok(hashes)
}

#[hdk_extern]
pub fn get_all_posts_sorted_by_votes(_: ()) -> ExternResult<Vec<ActionHash>> {
    let path = Path::from("all_posts");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllPosts, None)?;

    // Get all posts metadata and sort by votes, descending
    let mut posts_metadata: Vec<PostMetadata> = links
        .into_iter()
        .filter_map(|link| ActionHash::try_from(link.target).ok())
        .filter_map(|target| get_post_metadata(target).ok())
        .collect();
    posts_metadata.sort_by(|a, b| -> Ordering {
        let a_value = (a.upvotes - a.downvotes) as isize;
        let b_value = (b.upvotes - b.downvotes) as isize;

        if a_value == b_value {
            // If votes equal, order by record timestamp, asc
            return a
                .record
                .action()
                .timestamp()
                .cmp(&b.record.action().timestamp());
        } else {
            // Order by highest value, desc
            return b_value.cmp(&a_value);
        }
    });

    // Return hashes
    let hashes: Vec<ActionHash> = posts_metadata
        .into_iter()
        .map(|r| r.record.action_address().clone())
        .collect();

    Ok(hashes)
}
