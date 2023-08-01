use crate::*;
use hdi::prelude::*;

pub fn validate_link_authored_by_base(
    action: CreateLink,
    base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let author_hash = AnyLinkableHash::try_from(action.author).map_err(|e| wasm_error!(e))?;

    if author_hash != base {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "AgentToVotePost must be authored by linked agent",
        )));
    }
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_link_deleted_by_author(
    action: DeleteLink,
    original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    if !action.author.eq(&original_action.author) {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "AgentToVotePost must be authored by linked agent",
        )));
    }
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_vote_by_agent(
    action: CreateLink,
    _base: AnyLinkableHash,
    target: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let author_hash = AnyLinkableHash::try_from(action.clone().author).map_err(|e| wasm_error!(e))?;
    if author_hash != target {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "AgentToVotePost must be authored by linked agent",
        )));
    }
    let vote_tag_bytes = SerializedBytes::from(UnsafeBytes::from(tag.0));
    let vote_tag: VoteTag = vote_tag_bytes.try_into().map_err(|_e| {
        wasm_error!(WasmErrorInner::Guest(
            "failed to deserialize bytes to VoteTag".into()
        ))
    })?;

    // TODO: make this a DNA property
    // Value must be -1 or 1
    if vote_tag.value != -1 && vote_tag.value != 1 {
        return Ok(ValidateCallbackResult::Invalid(String::from(
            "VotePostToAgent tag value must be 1 or -1",
        )));
    }

    Ok(ValidateCallbackResult::Valid)
}
