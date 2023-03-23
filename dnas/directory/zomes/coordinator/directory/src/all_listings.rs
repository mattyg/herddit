use hdk::prelude::{*, holo_hash::DnaHash};
use directory_integrity::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct GetListingsInput {
    include_public: bool,
    include_private: bool,
}

#[hdk_extern]
pub fn get_listings(input: GetListingsInput) -> ExternResult<Vec<(DnaHash, ActionHash)>> {
    let mut records: Vec<Record> = vec!();

    if input.include_public {
        let mut public_listing_records = get_public_listings()?;
        records.append(&mut public_listing_records);
    }

    if input.include_private {
        let mut private_listing_records = query_private_listings()?;
        records.append(&mut private_listing_records);
    }

    records.sort_by_key(|r| r.action_hashed().timestamp());

    let listing_dna_hashes: Vec<DnaHash> = records.clone()
        .into_iter()
        .filter_map(|r| r.entry.to_app_option().ok())
        .filter_map(|r| r)
        .map(|listing: Listing| listing.dna)
        .collect();

    let listing_action_hashes: Vec<ActionHash> = records
        .into_iter()
        .map(|r| r.action_address().clone())
        .collect();

    let hashes = listing_dna_hashes
        .iter()
        .zip(listing_action_hashes.iter())
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();

    Ok(hashes)
}

fn get_public_listings() -> ExternResult<Vec<Record>> {
    let path = Path::from("all_listings");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllListings, None)?;

    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();

    let records: Vec<Record> = HDK.with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .flatten()
        .collect();

    Ok(records)
}

pub fn query_private_listings() -> ExternResult<Vec<Record>> {
    let private_listing_entry_type: EntryType = UnitEntryTypes::PrivateListing.try_into()?;
    let filter = ChainQueryFilter::new()
        .entry_type(private_listing_entry_type)
        .include_entries(true);

    let records = query(filter)?;

    Ok(records)
}