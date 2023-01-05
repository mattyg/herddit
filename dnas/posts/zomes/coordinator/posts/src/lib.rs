pub mod my_posts;
pub mod all_posts;
pub mod post;
use hdk::prelude::*;
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}
