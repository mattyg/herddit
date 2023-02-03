use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Listing {
    pub title: String,
    pub description: String,
    pub network_seed: String,
    pub dna: DnaHash
}
pub fn validate_create_listing(
    _action: EntryCreationAction,
    _listing: Listing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_listing(
    _action: Update,
    _listing: Listing,
    _original_action: EntryCreationAction,
    _original_listing: Listing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_listing(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_listing: Listing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_listing_updates(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_listing_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ListingUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_listings(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_listings(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllListings links cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_home_listing(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO:  only DNA progenitor can define home listing
    // TODO:  only one link of this type should exist (not possible?)
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_home_listing(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("HomeListing links cannot be deleted"),
        ),
    )
}
