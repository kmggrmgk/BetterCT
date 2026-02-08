use crate::ResourceMetadata;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use blake3::Hasher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub parent_ids: Vec<String>,
    pub message: String,
    pub timestamp: i64,
    pub resource: HashMap<String, ResourceMetadata>,
    pub author: String,
}

#[derive(Debug)]
pub struct VersionManager {
    version: HashMap<String, Version>,
    branches: HashMap<String, String>, // branch_name -> version_id
}

impl VersionManager {
    pub fn new() -> Self {
        let mut version = HashMap::new();
        let mut branches = HashMap::new();

        // Create initial version
        let initial_version = Version {
            id: "initial".to_string(),
            parent_ids: Vec::new(),
            message: "Initial version".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            resource: HashMap::new(),
            author: "System".to_string(),
        };

        version.insert(initial_version.id.clone(), initial_version.clone());
        branches.insert("main".to_string(), initial_version.id);

        Self {
            version,
            branches,
        }
    }

    pub fn create_version(
        &mut self,
        parent_ids: Vec<String>,
        message: String,
        resources: HashMap<String, ResourceMetadata>,
        author: &str,
    ) -> Result<String> {
        let id = self.generate_version_id(&parent_ids, &message, &resources);

        let version = Version {
            id: id.clone(),
            parent_ids,
            message,
            timestamp: chrono::Utc::now().timestamp(),
            resource: resources,
            author: author.to_string(),
        };

        self.version.insert(id.clone(), version);
        Ok(id)
    }

    fn generate_version_id(
        &self,
        parent_ids: &[String],
        message: &str,
        resources: &HashMap<String, ResourceMetadata>,
    ) -> String {
        let mut hasher = Hasher::new();

        for parent_id in parent_ids {
            hasher.update(parent_id.as_bytes());
        }

        hasher.update(message.as_bytes());

        let mut resource_keys: Vec<&String> = resources.keys().collect();
        resource_keys.sort();

        for key in resource_keys {
            hasher.update(key.as_bytes());
            if let Some(metadata) = resources.get(key) {
                // Convert metadata to bytes for hashing
                if let Ok(serialized) = serde_json::to_vec(metadata) {
                    hasher.update(&serialized);
                }
            }
        }

        hasher.finalize().to_string()
    }

    pub fn get_version(&self, version_id: &str) -> Option<&Version> {
        self.version.get(version_id)
    }

    pub fn get_branch_head(&self, branch_name: &str) -> Option<&Version> {
        self.branches
            .get(branch_name)
            .and_then(|version_id| self.version.get(version_id))
    }

    pub fn checkout_branch(&mut self, branch_name: &str, version_id: &str) -> bool {
        if self.version.contains_key(version_id) {
            self.branches.insert(branch_name.to_string(), version_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn create_branch(&mut self, new_branch: &str, from_branch: &str) -> bool {
        if let Some(version_id) = self.branches.get(from_branch) {
            self.branches.insert(new_branch.to_string(), version_id.clone());
            true
        } else { 
            false
        }
    }
    
    pub fn merge_versions(&mut self, base_id: &str, other_id: &str) -> Result<HashMap<String, ResourceMetadata>> {
        let base_version = self.version.get(base_id)
            .ok_or_else(|| anyhow::anyhow!("Base version not found"))?;
        
        let other_version = self.version.get(other_id)
            .ok_or_else(|| anyhow::anyhow!("Other version not found"))?;
        
        let mut merged = base_version.resource.clone();
        
        // Simple merge strategy: take the latest version of each resource
        for (key, resource) in &other_version.resource {
            merged.insert(key.clone(), resource.clone());
        }
        
        Ok(merged)
    }
    
    pub fn get_version_history(&self, version_id: &str) -> Vec<Version> {
        let mut history = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![version_id.to_string()];

        while let Some(current_id) = stack.pop() {
            if visited.contains(&current_id) {
                continue;
            }
            
            visited.insert(current_id.clone());
            
            if let Some(version) = self.version.get(&current_id) {
                history.push(version.clone());
                
                for parent_id in &version.parent_ids {
                    stack.push(parent_id.clone());
                }
            }
        }
        
        history.sort_by_key(|v| v.timestamp);
        history
    }
}