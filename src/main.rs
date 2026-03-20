use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};

use compliance_profiles::error::Result;
use compliance_profiles::hasher;
use compliance_profiles::index::ProfileIndex;
use compliance_profiles::known_profiles;

#[derive(Parser)]
#[command(name = "compliance-profiles")]
#[command(about = "Compliance profile index with government/community InSpec profile catalog")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all indexed profiles.
    List {
        /// Filter by provider (e.g., aws, gcp, azure, kubernetes).
        #[arg(long)]
        provider: Option<String>,
        /// Filter by NIST 800-53 family (e.g., AC, CM, SC).
        #[arg(long)]
        nist: Option<String>,
    },
    /// Hash a profile directory using BLAKE3.
    Hash {
        /// Path to the profile directory.
        path: String,
    },
    /// Generate the profile index from a profiles directory.
    Index {
        /// Path to profiles root directory.
        #[arg(default_value = "profiles")]
        profiles_dir: String,
        /// Output path for index.json.
        #[arg(short, long, default_value = "index.json")]
        output: String,
    },
    /// Show statistics for known profiles.
    Stats,
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List { provider, nist } => {
            let mut index = ProfileIndex::new();
            for profile in known_profiles() {
                // Ignore duplicate errors when loading known profiles.
                let _ = index.add_profile(profile);
            }

            let profiles: Vec<_> = if let Some(ref prov) = provider {
                index
                    .find_by_provider(prov)
                    .into_iter()
                    .cloned()
                    .collect()
            } else if let Some(ref family) = nist {
                index
                    .find_by_nist_family(family)
                    .into_iter()
                    .cloned()
                    .collect()
            } else {
                index.profiles.clone()
            };

            for profile in &profiles {
                println!(
                    "{:<45} {:>4} controls  [{}]  {}",
                    profile.id,
                    profile.control_count,
                    profile.source,
                    profile
                        .applicable_providers
                        .join(", ")
                );
            }
            println!("\n{} profiles listed.", profiles.len());
        }
        Command::Hash { path } => {
            let hash = hasher::hash_profile_directory(Path::new(&path))?;
            println!("{hash}  {path}");
        }
        Command::Index {
            profiles_dir,
            output,
        } => {
            let profiles_path = Path::new(&profiles_dir);
            let mut index = ProfileIndex::new();

            if profiles_path.is_dir() {
                for entry in std::fs::read_dir(profiles_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        let inspec_yml = path.join("inspec.yml");
                        if inspec_yml.exists() {
                            let id = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();

                            let control_count = count_controls(&path);
                            let hash = hasher::hash_profile_directory(&path)?;

                            let entry = compliance_profiles::profile::ProfileEntry {
                                id: id.clone(),
                                name: id.clone(),
                                source: compliance_profiles::profile::ProfileSource::Custom,
                                upstream_url: String::new(),
                                upstream_ref: String::new(),
                                control_count,
                                profile_hash: hash,
                                applicable_providers: vec![],
                                nist_families: vec![],
                                cis_benchmarks: vec![],
                                rspec_generated: false,
                                last_synced: None,
                            };
                            let _ = index.add_profile(entry);
                        }
                    }
                }
            }

            index.compute_stats();
            index.save(Path::new(&output))?;
            println!(
                "Index written to {output} ({} profiles, {} controls)",
                index.len(),
                index.total_controls()
            );
        }
        Command::Stats => {
            let profiles = known_profiles();
            let total_controls: usize = profiles.iter().map(|p| p.control_count).sum();
            let total_rspec = profiles.iter().filter(|p| p.rspec_generated).count();

            let mut providers = std::collections::BTreeSet::new();
            let mut nist = std::collections::BTreeSet::new();
            for profile in &profiles {
                for p in &profile.applicable_providers {
                    providers.insert(p.as_str());
                }
                for f in &profile.nist_families {
                    nist.insert(f.as_str());
                }
            }

            println!("Compliance Profile Statistics");
            println!("=============================");
            println!("Total profiles:       {}", profiles.len());
            println!("Total controls:       {total_controls}");
            println!("RSpec generated:      {total_rspec}");
            println!(
                "Providers covered:    {}",
                providers
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            println!(
                "NIST families:        {}",
                nist.into_iter().collect::<Vec<_>>().join(", ")
            );
        }
    }

    Ok(())
}

/// Count InSpec control files (*.rb) in the controls/ subdirectory.
fn count_controls(profile_dir: &Path) -> usize {
    let controls_dir = profile_dir.join("controls");
    if !controls_dir.is_dir() {
        return 0;
    }

    std::fs::read_dir(controls_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .is_some_and(|ext| ext == "rb")
                })
                .count()
        })
        .unwrap_or(0)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
