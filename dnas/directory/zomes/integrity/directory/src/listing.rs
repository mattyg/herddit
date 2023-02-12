use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, Eq, PartialEq)]
pub struct Listing {
    pub title: String,
    pub description: String,
    pub network_seed: String,
    pub dna: DnaHash,
}

#[hdk_entry_helper]
#[derive(Clone, Eq, PartialEq)]
pub struct PrivateListing {
    pub title: String,
    pub description: String,
    pub network_seed: String,
    pub dna: DnaHash,
}

pub fn validate_create_listing(
    _action: EntryCreationAction,
    listing: Listing
) -> ExternResult<ValidateCallbackResult> {
    // Listing title < 50 characters
    if listing.title.chars().count() > 50 {
        return Ok(ValidateCallbackResult::Invalid("Title must be < 50 characters".into()));
    }

    // Listing title contains no whitespace, is alphanumeric
    if !listing.title.chars().all(|c| !char::is_whitespace(c) && char::is_alphanumeric(c)) {
        return Ok(
            ValidateCallbackResult::Invalid(
                "Title can only contain alphanumeric characters, and no spaces".into()
            )
        );
    }

    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_listing(
    _action: Update,
    _listing: Listing,
    _original_action: EntryCreationAction,
    _original_listing: Listing
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("Listings cannot be updated".into()))
}
pub fn validate_delete_listing(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_listing: Listing
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("Listings cannot be deleted".into()))
}
pub fn validate_create_link_listing_updates(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("Listings cannot be updated".into()))
}
pub fn validate_delete_link_listing_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("ListingUpdates links cannot be deleted")))
}
pub fn validate_create_link_all_listings(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_listings(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("AllListings links cannot be deleted")))
}