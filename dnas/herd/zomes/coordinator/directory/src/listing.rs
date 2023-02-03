use hdk::prelude::*;
use directory_integrity::*;
#[hdk_extern]
pub fn create_listing(listing: Listing) -> ExternResult<Record> {
    let listing_hash = create_entry(&EntryTypes::Listing(listing.clone()))?;
    let record = get(listing_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Listing"))
            ),
        )?;
        
    // Link from all_listings -> Listing ActionHash
    let path = Path::from("all_listings");
    create_link(
        path.path_entry_hash()?,
        listing_hash.clone(),
        LinkTypes::AllListings,
        (),
    )?;

    Ok(record)
}

#[hdk_extern]
pub fn get_listing(original_listing_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_listing_hash.clone(),
        LinkTypes::ListingUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
    let latest_listing_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_listing_hash.clone(),
    };
    get(latest_listing_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateListingInput {
    pub original_listing_hash: ActionHash,
    pub previous_listing_hash: ActionHash,
    pub updated_listing: Listing,
}
#[hdk_extern]
pub fn update_listing(input: UpdateListingInput) -> ExternResult<Record> {
    let updated_listing_hash = update_entry(
        input.previous_listing_hash.clone(),
        &input.updated_listing,
    )?;
    create_link(
        input.original_listing_hash.clone(),
        updated_listing_hash.clone(),
        LinkTypes::ListingUpdates,
        (),
    )?;
    let record = get(updated_listing_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Listing"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_listing(original_listing_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_listing_hash)
}
