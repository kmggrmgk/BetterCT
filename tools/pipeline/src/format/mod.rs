pub mod texture;
pub mod model3d;
pub mod audio;

pub use texture::*;
pub use model3d::*;
pub use audio::*;

use crate::ResourceMetadata;
use anyhow::Result;

pub trait FormatProcessor {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn get_metadata(&self, data: &[u8]) -> Result<ResourceMetadata>;
    fn validate(&self, data: &[u8]) -> Result<bool>;
}