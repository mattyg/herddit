use hdk::prelude::*;

pub fn get_latest_record(original_ah: ActionHash) -> ExternResult<Record> {
  let details = get_details(original_ah, GetOptions::latest())?.ok_or(wasm_error!(WasmErrorInner::Guest("Record with that original action hash not found".into())))?;

  match details {
      Details::Record(record_details) => {
          match record_details.deletes.len() {
              // No deletes exist
              0 => {
                  match record_details.updates.len() {
                      // No updates exist, select original record
                      0 => Ok(record_details.record),
      
                      // Updates exist, select latest record
                      _ => {
                          let mut updates = record_details.updates.clone();
                          updates.sort_by_key(|a| a.hashed.content.timestamp());
                          let latest_update = updates.last().ok_or(wasm_error!(WasmErrorInner::Guest("Latest update not found".into())))?;
                      
                          let latest_update_record = get(latest_update.clone().hashed.hash, GetOptions::latest())?.ok_or(wasm_error!(WasmErrorInner::Guest("Latest update record not found".into())))?;
      
                          Ok(latest_update_record)
                      }
                  }
              }

              // Deletes exist, select earliest delete
              _ => {
                  let mut deletes = record_details.deletes.clone();
                  deletes.sort_by_key(|a| a.hashed.content.timestamp());
                  let earliest_delete = deletes.first().ok_or(wasm_error!(WasmErrorInner::Guest("Earliest delete record not found".into())))?;
              
                  let earliest_delete_record = get(earliest_delete.clone().hashed.hash, GetOptions::latest())?.ok_or(wasm_error!(WasmErrorInner::Guest("Earliest delete record not found".into())))?;

                  Ok(earliest_delete_record)
              }
          }
      }
      _ => Err(wasm_error!(WasmErrorInner::Guest("Expected AcitonHash".into())))
  }
}