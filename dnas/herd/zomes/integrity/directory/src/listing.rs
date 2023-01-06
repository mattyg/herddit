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
    action: EntryCreationAction,
    listing: Listing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_listing(
    action: Update,
    listing: Listing,
    original_action: EntryCreationAction,
    original_listing: Listing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_listing(
    action: Delete,
    original_action: EntryCreationAction,
    original_listing: Listing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_listing_updates(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_listing_updates(
    action: DeleteLink,
    original_action: CreateLink,
    base: AnyLinkableHash,
    target: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ListingUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_listings(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_listings(
    action: DeleteLink,
    original_action: CreateLink,
    base: AnyLinkableHash,
    target: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllListings links cannot be deleted"),
        ),
    )
}

pub fn validate_create_link_home_listing(
    action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO:  only DNA progenitor can define home listing
    // TODO:  only one link of this type should exist (not possible?)
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_home_listing(
    action: DeleteLink,
    original_action: CreateLink,
    base: AnyLinkableHash,
    target: AnyLinkableHash,
    tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("HomeListing links cannot be deleted"),
        ),
    )
}
