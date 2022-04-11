//! Git wrappers

pub mod add;
pub mod commit;
pub mod log;
pub mod push;
pub mod tag;

pub use add::git_add;
pub use commit::{git_commit, Commit};
pub use log::git_log;
pub use push::git_push;
pub use tag::{git_tag, Tag};
