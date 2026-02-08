use std::collections::HashMap;
use crate::storage::{ContentStore, ObjectType};
use crate::graph::{DependencyGraph, VersionManager};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use tracing::{error, info};


#[derive(Parser)]
#[command(name = "pipeline")]
#[command(about = "Git-like pipeline for multimedia assets", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, default_value = ".pipeline")]
    pub storage_path: PathBuf,

    #[arg(long, default_value = "info")]
    pub log_level: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new pipeline repository
    Init {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Store a resource
    Store {
        #[arg(short, long)]
        path: PathBuf,

        #[arg(long)]
        resource_type: Option<String>,
    },

    /// Retrieve a resource by hash
    Retrieve {
        hash: String,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Create a new version
    Commit {
        #[arg(short, long)]
        message: String,

        #[arg(long)]
        author: Option<String>,
    },

    /// Show version history
    Log {
        #[arg(long)]
        version_id: Option<String>,
    },

    /// Checkout a version or branch
    Checkout {
        target: String,
    },

    /// Create a new branch
    Branch {
        name: String,

        #[arg(long)]
        from: Option<String>,
    },

    /// Merge branches or versions
    Merge {
        source: String,
        target: String,
    },

    /// Show dependency graph
    Graph {
        #[arg(long)]
        hash: Option<String>,
    },

    /// Calculate diff between resources
    Diff {
        hash1: String,
        hash2: String,
    },
}

pub struct PipelineCli {
    store: ContentStore,
    dependency_graph: DependencyGraph,
    version_manager: VersionManager,
}

impl PipelineCli {
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        let store = ContentStore::new(storage_path)?;
        let dependency_graph = DependencyGraph::new();
        let version_manager = VersionManager::new();

        Ok(Self {
            store,
            dependency_graph,
            version_manager,
        })
    }

    pub async fn handle_command(&mut self, command: Commands) -> Result<()> {
        match command {
            Commands::Init {path} => {
                let path = path.unwrap_or_else(|| PathBuf::from("."));
                info!("Initializing project at {}", path.display());
                // Implementation here
            }

            Commands::Store {path, resource_type} => {
                info!("Storing resource at {}", path.display());
                let data = tokio::fs::read(&path).await?;

                // Determine resource type from file extension or parameter
                let _resource_type_str = resource_type.unwrap_or_else(|| {
                    "default".to_string()
                });

                // Store the object
                let object_type = ObjectType::Blob;
                let hash = self.store.store_object(object_type, data)?;

                info!("Stored {} as {}", path.display(), hash);
            }

            Commands::Retrieve {hash, output} => {
                info!("Retrieving {}", hash);
                let object = self.store.retrieve_object(&hash)?;

                let output_path = output.unwrap_or_else(|| PathBuf::from(&hash));
                tokio::fs::write(&output_path, object.data).await?;

                info!("Wrote {} to {}", hash, output_path.display());
            }

            Commands::Commit {message, author} => {
                let author = author.unwrap_or_else(|| "anonymous".to_string());
                info!("Committing with message: {} and author: {}", message, author);

                // Get current resources
                let resources = HashMap::new(); // This should gather current staged resources

                // Get current branch head
                let parent_id = self.version_manager.get_branch_head("main")
                    .map(|v| vec![v.id.clone()])
                    .unwrap_or_default();

                let version_id = self.version_manager.create_version(
                    parent_id,
                    message,
                    resources,
                    &author,
                )?;

                // Update branch
                self.version_manager.checkout_branch("main", &version_id);

                info!("Created commit: {}", version_id);
            }

            Commands::Log {version_id} => {
                if let Some(ref vid) = version_id {
                    let history = self.version_manager.get_version_history(&vid);
                    for version in history {
                        println!("{}: {} - {}",
                                 version_id.as_ref().unwrap_or(&"unknown".to_string()),
                                 version.author,
                                 version.message);
                    }
                } else {
                    if let Some(head) = self.version_manager.get_branch_head("main") {
                        let history = self.version_manager.get_version_history(&head.id);
                        for version in history {
                            println!("{}: {} - {}", head.id, version.author, version.message);
                        }
                    }
                }
            }

            Commands::Checkout {target} => {
                info!("Checking out: {}", target);
                // TODO: Implement
            }

            Commands::Branch {name, from} => {
                let from_branch = from.unwrap_or_else(|| "main".to_string());
                if self.version_manager.create_branch(&name, &from_branch) {
                    info!("Created branch: {} from {}", name, from_branch);
                } else {
                    error!("Failed to create branch: {}", name);
                }
            }

            Commands::Merge {source, target} => {
                info!("Merging {} into {}", source, target);
                // TODO: Implement merge
            }

            Commands::Graph {hash} => {
                if let Some(h) = hash {
                    let deps = self.dependency_graph.get_dependencies(&h);
                    println!("Dependencies for {}:", h);
                    for dep in deps {
                        println!(" - {}", dep.hash);
                    }
                } else {
                    println!("Dependency graph has {} nodes", self.dependency_graph.node_indices.len());
                }
            }

            Commands::Diff {hash1, hash2} => {
                info!("Calculating diff between {} and {}", hash1, hash2);
                let obj1 = self.store.retrieve_object(&hash1)?;
                let obj2 = self.store.retrieve_object(&hash2)?;

                let diff = crate::storage::DiffEngine::compute_binary_diff(
                    &obj1.data,
                    &obj2.data,
                );

                println!("Found {} differences", diff.len());
                for (pos, len, _) in diff {
                    println!("At position {}: {} bytes changed", pos, len);
                }
            }
        }

        Ok(())
    }
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(match cli.log_level.as_str() {
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "info" => tracing::Level::INFO,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => tracing::Level::INFO,
        })
        .init();

    let mut pipeline = PipelineCli::new(cli.storage_path)?;
    pipeline.handle_command(cli.command).await?;

    Ok(())
}