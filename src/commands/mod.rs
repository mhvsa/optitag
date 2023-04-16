pub mod clear;
pub mod get;
pub mod query;
pub mod tag;
pub mod untag;

pub use clear::Clear;
pub use get::Get;
pub use query::Query;
pub use tag::Tag;
pub use untag::Untag;

pub trait Command {
    fn execute(&self, tree: &sled::Tree) -> Result<String, String>;
}