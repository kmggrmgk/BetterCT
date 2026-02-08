use crate::ResourceMetadata;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Dfs, EdgeRef};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub hash: String,
    pub metadata: ResourceMetadata,
}

#[derive(Debug)]
pub struct DependencyGraph {
    graph: DiGraph<ResourceNode, ()>,
    pub node_indices: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }
    
    pub fn add_resource(&mut self, metadata: ResourceMetadata) -> NodeIndex {
        let node = ResourceNode {
            hash: metadata.hash.clone(),
            metadata: metadata.clone(),
        };
        
        let index = self.graph.add_node(node);
        self.node_indices.insert(metadata.hash.clone(), index);
        index
    }
    
    pub fn add_dependency(&mut self, from_hash: &str, to_hash: &str) -> bool {
        if let (Some(&from_idx), Some(&to_idx)) = (
            self.node_indices.get(from_hash),
            self.node_indices.get(to_hash),
        ) {
            self.graph.add_edge(from_idx, to_idx, ());
            true
        } else { 
            false
        }
    }
    
    pub fn get_dependencies(&self, hash: &str) -> Vec<ResourceNode> {
        let mut dependencies = Vec::new();
        
        if let Some(&node_idx) = self.node_indices.get(hash) {
            for neighbor in self.graph.neighbors(node_idx) {
                if let Some(node) = self.graph.node_weight(neighbor) {
                    dependencies.push(node.clone());
                }
            }
        }
        
        dependencies
    }
    
    pub fn get_dependents(&self, hash: &str) -> Vec<ResourceNode> {
        let mut dependents = Vec::new();
        
        if let Some(&node_idx) = self.node_indices.get(hash) {
            for edge in self.graph.edges_directed(node_idx, petgraph::Direction::Incoming) {
                if let Some(source) = self.graph.node_weight(edge.source()) {
                    dependents.push(source.clone());
                }
            }
        }
        
        dependents
    }
    
    pub fn find_circular_dependencies(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashMap::new();
        
        for node_idx in self.graph.node_indices() {
            let mut dfs = Dfs::new(&self.graph, node_idx);
            let mut path = Vec::new();
            
            while let Some(nx) = dfs.next(&self.graph) {
                if visited.contains_key(&nx) {
                    if let Some(pos) = path.iter().position(|&x| x == nx) {
                        let cycle: Vec<String> = path[pos..]
                            .iter()
                            .filter_map(|&idx| self.graph.node_weight(idx).map(|n| n.hash.clone()))
                            .collect();
                        
                        if !cycle.is_empty() {
                            cycles.push(cycle);
                        }
                    }
                    break;
                }
                
                visited.insert(nx, true);
                path.push(nx);
            }
        }
        
        cycles
    }
    
    pub fn topological_sort(&self) -> Vec<ResourceNode> {
        let mut order = Vec::new();
        let mut visited = HashMap::new();
        
        for node_idx in self.graph.node_indices() {
            self.dfs(node_idx, &mut visited, &mut order);
        }
        
        order.reverse();
        order
    }
    
    fn dfs(&self, node_idx: NodeIndex, visited: &mut HashMap<NodeIndex, bool>, order: &mut Vec<ResourceNode>) {
        if visited.contains_key(&node_idx) {
            return;
        }
        
        visited.insert(node_idx, true);
        
        for neighbor in self.graph.neighbors(node_idx) {
            self.dfs(neighbor, visited, order);
        }
        
        if let Some(node) = self.graph.node_weight(node_idx) {
            order.push(node.clone());
        }
    }
}