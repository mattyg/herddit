use hdi::prelude::*;
use crate::*;

pub fn validate_create_link_my_upvoted_posts(
    action: CreateLink,
    base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if !AnyLinkableHash::from(action.author).eq(&base) {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("AgentToVotePost must be authored by linked agent"),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_my_upvoted_posts(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if !action.author.eq(&original_action.author) {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("AgentToVotePost must be authored by linked agent"),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_vote_post_by_agent(
    action: CreateLink,
    base: AnyLinkableHash,
    target: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if !AnyLinkableHash::from(action.clone().author).eq(&target) {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("AgentToVotePost must be authored by linked agent"),
            ),
        );
    }
    let post_vote_bytes = SerializedBytes::from(UnsafeBytes::from(tag.0));
    let post_vote: PostVoteTag = post_vote_bytes
        .try_into()
        .map_err(|_e| {
            wasm_error!(
                WasmErrorInner::Guest("failed to deserialize bytes to PostVoteTag"
                .into())
            )
        })?;
    if post_vote.value != -1 && post_vote.value != 1 {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("VotePostToAgent tag value must be 1 or -1"),
            ),
        );
    }
    let agent_activity = must_get_agent_activity(
        action.author,
        ChainFilter {
            chain_top: action.prev_action,
            filters: ChainFilters::ToGenesis,
            include_cached_entries: false,
        },
    )?;
    let previous_links: Vec<RegisterAgentActivity> = agent_activity
        .into_iter()
        .filter(|activity| {
            activity.action.hashed.action_type().eq(&ActionType::CreateLink)
        })
        .filter(|activity| -> bool {
            let create_link = CreateLink::try_from(activity.action.action().clone())
                .unwrap();
            let link_type = LinkTypes::from_type(
                    action.zome_index,
                    create_link.link_type,
                )
                .unwrap()
                .unwrap();
            link_type.eq(&LinkTypes::PostVoteByAgent)
                && create_link.base_address.eq(&base)
        })
        .collect();
    if previous_links.len() >= 5 {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("Agent has already voted on this post 5 times!"),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_vote_post_by_agent(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if action.author.ne(&original_action.author) {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("VotePostToAgent must be authored by linked agent"),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}
