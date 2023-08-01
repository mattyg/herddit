use hdk::prelude::*;
use posts_integrity::*;
#[hdk_extern]
pub fn get_my_posts(author: AgentPubKey) -> ExternResult<Vec<ActionHash>> {
    let links = get_links(author, LinkTypes::MyPosts, None)?;
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
