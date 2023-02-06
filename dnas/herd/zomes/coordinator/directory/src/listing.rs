use hdk::prelude::*;
use directory_integrity::*;

#[hdk_extern]
pub fn create_private_listing(listing: PrivateListing) -> ExternResult<ActionHash> {
    let listing_hash = create_entry(EntryTypes::PrivateListing(listing.clone()))?;

    Ok(listing_hash)
}

#[hdk_extern]
pub fn create_listing(listing: Listing) -> ExternResult<ActionHash> {
    let listing_hash = create_entry(&EntryTypes::Listing(listing.clone()))?;
            
    let path = Path::from("all_listings");
    create_link(
        path.path_entry_hash()?,
        listing_hash.clone(),
        LinkTypes::AllListings,
        (),
    )?;

    Ok(listing_hash)
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

#[hdk_extern]
pub fn listing_to_bubble_babble(listing: Listing) -> ExternResult<String> {
    let bytes = SerializedBytes::try_from(listing).map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    let babble = boba::encode(bytes.bytes());

    Ok(babble)
}

#[hdk_extern]
pub fn bubble_babble_to_listing(babble: String) -> ExternResult<Listing> {
    let bytes = boba::decode(babble).map_err(|_| wasm_error!(WasmErrorInner::Guest("Failed to decode bytes from provided bubble babble".into())))?;
    let serialized_bytes = SerializedBytes::try_from(UnsafeBytes::from(bytes))?;
    let listing = Listing::try_from(serialized_bytes).map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    
    Ok(listing)
}