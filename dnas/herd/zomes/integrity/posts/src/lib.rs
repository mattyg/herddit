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

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Comment {
    pub content: String,
    pub post_ah: ActionHash,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct VoteTag {
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
    // Posts
    PostUpdates,
    AllPosts,
    MyPosts,

    // Post votes
    MyVotedPosts,
    PostVoteByAgent,

    // Comments
    PostToComments,
    CommentUpdates,
    MyComments,

    // Comment votes
    MyVotedComments,
    CommentVoteByAgent,
}

#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}