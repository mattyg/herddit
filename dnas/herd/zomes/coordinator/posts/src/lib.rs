pub mod all_posts;
pub mod comment;
pub mod my_posts;
pub mod post;
pub mod utils;
use hdk::prelude::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}
