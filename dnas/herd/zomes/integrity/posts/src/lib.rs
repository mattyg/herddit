pub mod comment;
pub use comment::*;
pub mod post;
pub use post::*;
pub mod votes;
pub use votes::*;
pub mod validate;
pub use validate::*;
use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Post {
    pub title: String,
    pub content: String,
}
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct PostVoteTag {
    pub value: i8,
}
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Post(Post),
    Comment(Comment),
}
#[hdk_link_types]
pub enum LinkTypes {
    PostUpdates,
    AllPosts,
    MyPosts,
    MyUpvotedPosts,
    PostVoteByAgent,
    PostToComments,
    CommentUpdates,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
