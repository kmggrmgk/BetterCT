use crate::ResourceMetadata;
use crate::ResourceType;
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Model3DProcessor;

impl Model3DProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_gltf(&self, data: &[u8]) -> Result<bool> {
        match gltf::Gltf::from_slice(data) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn get_scene_info(&self, data: &[u8]) -> Result<(usize, usize, usize)> {
        let gltf = gltf::Gltf::from_slice(data)?;
        let scene_count = gltf.scenes().count();
        let mesh_count = gltf.meshes().count();
        let node_count = gltf.nodes().count();

        Ok((scene_count, mesh_count, node_count))
    }
}

impl super::FormatProcessor for Model3DProcessor {
    fn process(&self, data: &[u8]) -> Result<Vec<u8>> {
        // For now, just validate and return the same data
        self.validate_gltf(data)?;
        Ok(data.to_vec())
    }

    fn get_metadata(&self, data: &[u8]) -> Result<ResourceMetadata> {
        let (scenes, meshes, nodes) = self.get_scene_info(data)?;

        Ok(ResourceMetadata {
            hash: blake3::hash(data).to_string(),
            resource_type: ResourceType::Model3D,
            size: data.len() as u64,
            create_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs() as i64,
            dependencies: vec![
                format!("gltf:scenes={}", scenes),
                format!("gltf:meshes={}", meshes),
                format!("gltf:nodes={}", nodes),
            ],
        })
    }

    fn validate(&self, data: &[u8]) -> Result<bool> {
        self.validate_gltf(data)
    }
}