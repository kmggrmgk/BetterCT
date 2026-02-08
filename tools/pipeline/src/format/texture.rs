use crate::ResourceMetadata;
use crate::ResourceType;
use anyhow::{Context, Result};
use image::ImageFormat;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TextureProcessor;

impl TextureProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_format(&self, data: &[u8], target_format: ImageFormat) -> Result<Vec<u8>> {
        let img = image::load_from_memory(data)
            .with_context(|| "Failed to load image")?;

        let mut buffer = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buffer), target_format)
            .with_context(|| "Failed to convert image format")?;

        Ok(buffer)
    }

    pub fn resize(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        let img = image::load_from_memory(data)?;
        let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

        let mut buffer = Vec::new();
        resized.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)?;

        Ok(buffer)
    }

    pub fn generate_mipmaps(&self, data: &[u8], levels: u32) -> Result<Vec<Vec<u8>>> {
        let img = image::load_from_memory(data)?;
        let mut mipmaps = Vec::new();

        let mut current_width = img.width();
        let mut current_height = img.height();

        for _ in 0..levels {
            if current_width < 1 || current_height < 1 {
                break;
            }

            let resized = img.resize_exact(
                current_width,
                current_height,
                image::imageops::FilterType::Lanczos3,
            );

            let mut buffer = Vec::new();
            resized.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)?;
            mipmaps.push(buffer);

            current_width /= 2;
            current_height /= 2;
        }

        Ok(mipmaps)
    }

    pub fn get_dimensions(&self, data: &[u8]) -> Result<(u32, u32)> {
        let img = image::load_from_memory(data)?;
        Ok((img.width(), img.height()))
    }
}

impl super::FormatProcessor for TextureProcessor {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Default processing: convert to PNG if not already
        if let Ok(format) = image::guess_format(data) {
            if format == ImageFormat::Png {
                return Ok(data.to_vec());
            }
        }
        self.convert_format(data, ImageFormat::Png)
    }

    fn get_metadata(&self, data: &[u8]) -> Result<ResourceMetadata> {
        let (width, height) = self.get_dimensions(data)?;

        Ok(ResourceMetadata {
            hash: blake3::hash(data).to_string(),
            resource_type: ResourceType::Texture,
            size: data.len() as u64,
            create_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs() as i64,
            dependencies: vec![format!("texture:{}x{}", width, height)],
        })
    }

    fn validate(&self, data: &[u8]) -> Result<bool> {
        match image::load_from_memory(data) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}