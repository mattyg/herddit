use hdi::prelude::*;

pub fn validate_create_herd(
    action: EntryCreationAction,
    herd: Herd,
) -> ExternResult<ValidateCallbackResult> {

  Ok(ValidateCallbackResult::Valid)
}