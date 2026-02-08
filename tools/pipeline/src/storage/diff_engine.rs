
use rayon::prelude::*;
use std::cmp::min;

pub struct DiffEngine;

impl DiffEngine {
    pub fn compute_chunked_diff(old: &[u8], new: &[u8], chunk_size: usize) -> Vec<(usize, Vec<u8>)> {
        let chunks = new.par_chunks(chunk_size).enumerate().filter_map(|(i, chunk)| {
            let start = i * chunk_size;
            let end = min(start + chunk_size, new.len());

            if end > start && (start >= old.len() || &old[start..min(end, old.len())] != chunk) {
                Some((start, chunk.to_vec()))
            } else {
                None
            }
        }).collect();

        chunks
    }

    pub fn compute_binary_diff(old: &[u8], new: &[u8]) -> Vec<(usize, usize, Vec<u8>)> {
        let mut diffs = Vec::new();
        
        // Handle case where new is longer than old
        if new.len() > old.len() {
            // Find where they start to differ
            let mut i = 0;
            while i < old.len() && i < new.len() && old[i] == new[i] {
                i += 1;
            }
            
            // Replace the differing part and append the rest
            if i < old.len() {
                diffs.push((i, old.len() - i, new[i..].to_vec()));
            } else {
                // Just append the extra part
                diffs.push((old.len(), 0, new[old.len()..].to_vec()));
            }
        } 
        // Handle case where old is longer than new
        else if old.len() > new.len() {
            // Find where they start to differ
            let mut i = 0;
            while i < new.len() && old[i] == new[i] {
                i += 1;
            }
            
            // Replace the differing part and delete the excess
            if i < new.len() {
                diffs.push((i, old.len() - i, new[i..].to_vec()));
            } else {
                // Just delete the excess part
                diffs.push((new.len(), old.len() - new.len(), vec![]));
            }
        }
        // Same length
        else {
            let mut i = 0;
            while i < old.len() && old[i] == new[i] {
                i += 1;
            }
            
            if i < old.len() {
                // Find the end of the differing region
                let mut j = old.len() - 1;
                while j > i && old[j] == new[j] {
                    j -= 1;
                }
                
                diffs.push((i, j - i + 1, new[i..=j].to_vec()));
            }
        }

        diffs
    }

    pub fn apply_diff(base: &[u8], diff: &[(usize, usize, Vec<u8>)]) -> Vec<u8> {
        let mut result = base.to_vec();
        
        // Process operations in forward order to maintain correct positioning
        for &(pos, delete_len, ref insert_data) in diff {
            if pos <= result.len() {
                // First delete, then insert at the same position
                if delete_len > 0 && pos + delete_len <= result.len() {
                    result.drain(pos..pos + delete_len);
                }
                if !insert_data.is_empty() {
                    result.splice(pos..pos, insert_data.iter().cloned());
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    
    #[test]
    fn test_binary_diff() {
        let old = b"hello world";
        let new = b"hello there";
        
        let diff = DiffEngine::compute_binary_diff(old, new);
        let patched = DiffEngine::apply_diff(old, &diff);
        
        assert_eq!(patched, new);
    }
}