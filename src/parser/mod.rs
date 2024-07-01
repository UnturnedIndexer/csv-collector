use std::path::PathBuf;

mod asset;
pub use asset::Asset;

pub trait Parser<T> {
    fn parse<P: Into<PathBuf>>(path: P) -> anyhow::Result<T>;
}
