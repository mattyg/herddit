use hdk::prelude::*;
use directory_integrity::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct GetListingsInput {
    include_public: bool,
    include_private: bool,
}

#[hdk_extern]
pub fn get_listings(input: GetListingsInput) -> ExternResult<Vec<ActionHash>> {
    let mut records: Vec<Record> = vec!();

    if input.include_public {
        let mut public_listing_records = get_public_listings()?;
        records.append(&mut public_listing_records)
    }

    if input.include_private {
        let mut private_listing_records = query_private_listings()?;
        records.append(&mut private_listing_records)
    }

    records.sort_by_key(|r| r.action_hashed().timestamp());
   
    let hashes: Vec<ActionHash> = records
        .into_iter()
        .map(|r| r.action_address().clone())
        .collect();

    Ok(hashes)
}

fn get_public_listings() -> ExternResult<Vec<Record>> {
    let path = Path::from("all_listings");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllListings, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    
    let records: Vec<Record> = HDK.with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .flatten()
        .collect();

    Ok(records)
}

fn query_private_listings() -> ExternResult<Vec<Record>> { 
  let private_listing_entry_type: EntryType = UnitEntryTypes::PrivateListing.try_into()?;
  let filter = ChainQueryFilter::new()
    .entry_type(private_listing_entry_type)
    .include_entries(true);

  let my_private_listings = query(filter)?;

  Ok(my_private_listings)
}