use hdk::prelude::*;
use herd_integrity::HerdInfo;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
pub fn get_info(_: ()) -> ExternResult<HerdInfo> {
  let dna_properties = dna_info()?.properties;

  let herd_info = HerdInfo::try_from(dna_properties)
    .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.into())))?;
  
  Ok(herd_info)
}