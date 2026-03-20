use std::path::Path;

use compliance_profiles::hasher;
use compliance_profiles::index::ProfileIndex;
use compliance_profiles::known_profiles;
use compliance_profiles::profile::{ProfileEntry, ProfileSource};

fn samples_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("samples").leak()
}

fn linux_baseline_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("samples/linux-baseline")
        .leak()
}

fn ssh_baseline_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("samples/ssh-baseline")
        .leak()
}

// --- Sample profile parsing ---

#[test]
fn linux_baseline_has_inspec_yml() {
    assert!(linux_baseline_dir().join("inspec.yml").exists());
}

#[test]
fn linux_baseline_has_controls() {
    let controls_dir = linux_baseline_dir().join("controls");
    assert!(controls_dir.is_dir());
    let count = std::fs::read_dir(&controls_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rb"))
        .count();
    assert!(count >= 2, "expected at least 2 controls, got {count}");
}

#[test]
fn linux_baseline_controls_contain_control_blocks() {
    let controls_dir = linux_baseline_dir().join("controls");
    for entry in std::fs::read_dir(&controls_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().is_some_and(|ext| ext == "rb") {
            let content = std::fs::read_to_string(entry.path()).unwrap();
            assert!(
                content.contains("control '"),
                "file {:?} missing control block",
                entry.path()
            );
            assert!(
                content.contains("impact"),
                "file {:?} missing impact declaration",
                entry.path()
            );
        }
    }
}

#[test]
fn ssh_baseline_has_inspec_yml() {
    assert!(ssh_baseline_dir().join("inspec.yml").exists());
}

#[test]
fn ssh_baseline_has_controls() {
    let controls_dir = ssh_baseline_dir().join("controls");
    assert!(controls_dir.is_dir());
    let count = std::fs::read_dir(&controls_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rb"))
        .count();
    assert!(count >= 2, "expected at least 2 controls, got {count}");
}

#[test]
fn ssh_baseline_controls_have_nist_tags() {
    let controls_dir = ssh_baseline_dir().join("controls");
    for entry in std::fs::read_dir(&controls_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().is_some_and(|ext| ext == "rb") {
            let content = std::fs::read_to_string(entry.path()).unwrap();
            assert!(
                content.contains("tag nist:"),
                "file {:?} missing NIST tag",
                entry.path()
            );
        }
    }
}

// --- Hashing sample profiles ---

#[test]
fn hash_linux_baseline() {
    let hash = hasher::hash_profile_directory(linux_baseline_dir()).unwrap();
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn hash_ssh_baseline() {
    let hash = hasher::hash_profile_directory(ssh_baseline_dir()).unwrap();
    assert_eq!(hash.len(), 64);
}

#[test]
fn hash_sample_profiles_differ() {
    let hash_linux = hasher::hash_profile_directory(linux_baseline_dir()).unwrap();
    let hash_ssh = hasher::hash_profile_directory(ssh_baseline_dir()).unwrap();
    assert_ne!(hash_linux, hash_ssh);
}

// --- Known profiles ---

#[test]
fn known_profiles_has_at_least_ten_entries() {
    assert!(known_profiles().len() >= 10);
}

#[test]
fn known_profiles_covers_aws() {
    let profiles = known_profiles();
    let aws_profiles: Vec<_> = profiles
        .iter()
        .filter(|p| p.applicable_providers.contains(&"aws".to_string()))
        .collect();
    assert!(
        !aws_profiles.is_empty(),
        "no profiles cover AWS"
    );
}

#[test]
fn known_profiles_covers_kubernetes() {
    let profiles = known_profiles();
    let k8s_profiles: Vec<_> = profiles
        .iter()
        .filter(|p| {
            p.applicable_providers
                .contains(&"kubernetes".to_string())
        })
        .collect();
    assert!(
        !k8s_profiles.is_empty(),
        "no profiles cover Kubernetes"
    );
}

// --- Index generation from samples ---

#[test]
fn index_from_samples() {
    let mut index = ProfileIndex::new();

    for entry in std::fs::read_dir(samples_dir()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() && path.join("inspec.yml").exists() {
            let id = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let controls_dir = path.join("controls");
            let control_count = if controls_dir.is_dir() {
                std::fs::read_dir(&controls_dir)
                    .unwrap()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().is_some_and(|ext| ext == "rb"))
                    .count()
            } else {
                0
            };

            let hash = hasher::hash_profile_directory(&path).unwrap();

            let profile = ProfileEntry {
                id: id.clone(),
                name: id,
                source: ProfileSource::Custom,
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
            index.add_profile(profile).unwrap();
        }
    }

    assert_eq!(index.len(), 2);
    assert!(index.total_controls() >= 4);
}

#[test]
fn index_save_and_reload_from_samples() {
    let dir = tempfile::TempDir::new().unwrap();
    let output = dir.path().join("test-index.json");

    let mut index = ProfileIndex::new();
    for profile in known_profiles() {
        let _ = index.add_profile(profile);
    }
    index.compute_stats();
    index.save(&output).unwrap();

    let loaded = ProfileIndex::load(&output).unwrap();
    assert_eq!(loaded.len(), index.len());
    assert_eq!(loaded.total_controls(), index.total_controls());
}

// --- Filter tests ---

#[test]
fn filter_known_profiles_by_provider_aws() {
    let mut index = ProfileIndex::new();
    for profile in known_profiles() {
        let _ = index.add_profile(profile);
    }
    let aws = index.find_by_provider("aws");
    assert!(aws.len() >= 3, "expected at least 3 AWS profiles");
}

#[test]
fn filter_known_profiles_by_provider_gcp() {
    let mut index = ProfileIndex::new();
    for profile in known_profiles() {
        let _ = index.add_profile(profile);
    }
    let gcp = index.find_by_provider("gcp");
    assert!(!gcp.is_empty(), "expected at least 1 GCP profile");
}

#[test]
fn filter_known_profiles_by_nist_ac() {
    let mut index = ProfileIndex::new();
    for profile in known_profiles() {
        let _ = index.add_profile(profile);
    }
    let ac = index.find_by_nist_family("AC");
    assert!(
        ac.len() >= 5,
        "expected at least 5 profiles covering AC family, got {}",
        ac.len()
    );
}

#[test]
fn filter_known_profiles_by_nist_cm() {
    let mut index = ProfileIndex::new();
    for profile in known_profiles() {
        let _ = index.add_profile(profile);
    }
    let cm = index.find_by_nist_family("CM");
    assert!(!cm.is_empty(), "expected at least 1 CM profile");
}
