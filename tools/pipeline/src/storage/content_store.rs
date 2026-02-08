use crate::ResourceMetadata;
use anyhow::{Context, Result};
use bincode::{Decode, Encode};
use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Resource(ResourceMetadata),
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ContentObject {
    pub object_type: ObjectType,
    pub data: Vec<u8>,
    pub hash: String,
}

impl ContentObject {
    pub fn new(object_type: ObjectType, data: Vec<u8>) -> Self {
        let hash = Self::compute_hash(&data);
        Self {
            object_type,
            data,
            hash,
        }
    }

    pub fn compute_hash(data: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().to_string()
    }

    pub fn save_to_disk(&self, storage_path: &Path) -> Result<()> {
        let hash_dir = &self.hash[0..2];
        let hash_file = &self.hash[2..];

        let object_dir = storage_path.join(hash_dir);
        fs::create_dir_all(&object_dir)?;

        let object_path = object_dir.join(hash_file);
        let serialized = bincode::encode_to_vec(self, bincode::config::standard())?;

        fs::write(object_path, serialized)?;
        info!("Saved object to disk: {}", self.hash);
        Ok(())
    }

    pub fn load_from_disk(storage_path: &Path, hash: &str) -> Result<Self> {
        let hash_dir = &hash[0..2];
        let hash_file = &hash[2..];

        let object_path = storage_path.join(hash_dir).join(hash_file);
        let data = fs::read(&object_path)
            .with_context(|| format!("Failed to read object: {}", hash))?;

        let (object, _): (ContentObject, usize) = bincode::decode_from_slice(&data, bincode::config::standard())?;
        Ok(object)
    }
}

#[derive(Debug)]
pub struct ContentStore {
    storage_path: PathBuf,
    objects: HashMap<String, ContentObject>,
}

impl ContentStore {
    pub fn new(storage_path: impl AsRef<Path>) -> Result<Self> {
        let storage_path = storage_path.as_ref().to_path_buf();
        fs::create_dir_all(&storage_path)?;

        Ok(Self {
            storage_path,
            objects: HashMap::new(),
        })
    }

    pub fn store_object(&mut self, object_type: ObjectType, data: Vec<u8>) -> Result<String> {
        let object = ContentObject::new(object_type, data);
        let hash = object.hash.clone();

        object.save_to_disk(&self.storage_path)?;
        self.objects.insert(hash.clone(), object);

        Ok(hash)
    }

    pub fn retrieve_object(&self, hash: &str) -> Result<ContentObject> {
        if let Some(object) = self.objects.get(hash) {
            return Ok(object.clone());
        }

        ContentObject::load_from_disk(&self.storage_path, hash)
    }

    pub fn object_exists(&self, hash: &str) -> bool {
        let hash_dir = &hash[0..2];
        let hash_file = &hash[2..];

        self.storage_path
            .join(hash_dir)
            .join(hash_file)
            .exists()
    }

    pub fn get_storage_path(&self) -> &Path {
        &self.storage_path
    }
}