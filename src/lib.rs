pub mod error;
pub mod hasher;
pub mod index;
pub mod profile;
pub mod sync;

use profile::{ProfileEntry, ProfileSource};

/// Return the catalog of known government/community compliance profiles.
///
/// These profiles are not shipped as submodules in the initial commit --
/// they are metadata entries that describe upstream InSpec profiles for
/// indexing, hashing, and RSpec transpilation.
#[must_use]
pub fn known_profiles() -> Vec<ProfileEntry> {
    vec![
        ProfileEntry {
            id: "mitre-rhel9-stig".to_string(),
            name: "RHEL 9 STIG Baseline".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/redhat-enterprise-linux-9-stig-baseline"
                .to_string(),
            upstream_ref: "main".to_string(),
            control_count: 452,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "gcp".to_string(), "azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
                "SI".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-aws-foundations-cis".to_string(),
            name: "AWS Foundations CIS Benchmark".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/aws-foundations-cis-baseline".to_string(),
            upstream_ref: "main".to_string(),
            control_count: 56,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec!["CIS-AWS-1.4".to_string()],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-azure-foundations-cis".to_string(),
            name: "Azure Foundations CIS Benchmark".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/microsoft-azure-cis-foundations-baseline"
                .to_string(),
            upstream_ref: "main".to_string(),
            control_count: 100,
            profile_hash: String::new(),
            applicable_providers: vec!["azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec!["CIS-Azure-1.3".to_string()],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-k8s-cluster-stig".to_string(),
            name: "Kubernetes Cluster STIG Baseline".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/kubernetes-cluster-stig-baseline".to_string(),
            upstream_ref: "main".to_string(),
            control_count: 90,
            profile_hash: String::new(),
            applicable_providers: vec!["kubernetes".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "CM".to_string(),
                "SC".to_string(),
                "SI".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-k3s-cluster-stig".to_string(),
            name: "K3s Cluster STIG Baseline".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/k3s-cluster-stig-baseline".to_string(),
            upstream_ref: "main".to_string(),
            control_count: 90,
            profile_hash: String::new(),
            applicable_providers: vec!["kubernetes".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "CM".to_string(),
                "SC".to_string(),
                "SI".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "dev-sec-linux-baseline".to_string(),
            name: "Linux Baseline".to_string(),
            source: ProfileSource::DevSec,
            upstream_url: "https://github.com/dev-sec/linux-baseline".to_string(),
            upstream_ref: "master".to_string(),
            control_count: 53,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "gcp".to_string(), "azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "CM".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "dev-sec-ssh-baseline".to_string(),
            name: "SSH Baseline".to_string(),
            source: ProfileSource::DevSec,
            upstream_url: "https://github.com/dev-sec/ssh-baseline".to_string(),
            upstream_ref: "master".to_string(),
            control_count: 57,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "gcp".to_string(), "azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "dev-sec-cis-kubernetes-benchmark".to_string(),
            name: "CIS Kubernetes Benchmark".to_string(),
            source: ProfileSource::DevSec,
            upstream_url: "https://github.com/dev-sec/cis-kubernetes-benchmark".to_string(),
            upstream_ref: "master".to_string(),
            control_count: 120,
            profile_hash: String::new(),
            applicable_providers: vec!["kubernetes".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "CM".to_string(),
                "SC".to_string(),
                "SI".to_string(),
            ],
            cis_benchmarks: vec!["CIS-K8s-1.6".to_string()],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "dev-sec-cis-docker-benchmark".to_string(),
            name: "CIS Docker Benchmark".to_string(),
            source: ProfileSource::DevSec,
            upstream_url: "https://github.com/dev-sec/cis-docker-benchmark".to_string(),
            upstream_ref: "master".to_string(),
            control_count: 110,
            profile_hash: String::new(),
            applicable_providers: vec!["kubernetes".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "CM".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec!["CIS-Docker-1.3".to_string()],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "gcp-cis-benchmark".to_string(),
            name: "GCP CIS Benchmark".to_string(),
            source: ProfileSource::Gcp,
            upstream_url: "https://github.com/GoogleCloudPlatform/inspec-gcp-cis-benchmark"
                .to_string(),
            upstream_ref: "master".to_string(),
            control_count: 60,
            profile_hash: String::new(),
            applicable_providers: vec!["gcp".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec!["CIS-GCP-1.2".to_string()],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-windows-server-2019-stig".to_string(),
            name: "Windows Server 2019 STIG Baseline".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/microsoft-windows-server-2019-stig-baseline"
                .to_string(),
            upstream_ref: "main".to_string(),
            control_count: 380,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
                "SI".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-ubuntu-20-stig".to_string(),
            name: "Ubuntu 20.04 STIG Baseline".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/canonical-ubuntu-20.04-lts-stig-baseline"
                .to_string(),
            upstream_ref: "main".to_string(),
            control_count: 290,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "gcp".to_string(), "azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "dev-sec-nginx-baseline".to_string(),
            name: "Nginx Baseline".to_string(),
            source: ProfileSource::DevSec,
            upstream_url: "https://github.com/dev-sec/nginx-baseline".to_string(),
            upstream_ref: "master".to_string(),
            control_count: 22,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "gcp".to_string(), "azure".to_string()],
            nist_families: vec![
                "CM".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "dev-sec-postgres-baseline".to_string(),
            name: "PostgreSQL Baseline".to_string(),
            source: ProfileSource::DevSec,
            upstream_url: "https://github.com/dev-sec/postgres-baseline".to_string(),
            upstream_ref: "master".to_string(),
            control_count: 24,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string(), "gcp".to_string(), "azure".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
        ProfileEntry {
            id: "mitre-aws-rds-crunchy-postgres-stig".to_string(),
            name: "AWS RDS Crunchy PostgreSQL STIG Baseline".to_string(),
            source: ProfileSource::Mitre,
            upstream_url: "https://github.com/mitre/aws-rds-crunchy-data-postgresql-9-stig-baseline"
                .to_string(),
            upstream_ref: "main".to_string(),
            control_count: 78,
            profile_hash: String::new(),
            applicable_providers: vec!["aws".to_string()],
            nist_families: vec![
                "AC".to_string(),
                "AU".to_string(),
                "CM".to_string(),
                "IA".to_string(),
                "SC".to_string(),
            ],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_profiles_not_empty() {
        let profiles = known_profiles();
        assert!(!profiles.is_empty());
    }

    #[test]
    fn known_profiles_has_at_least_ten() {
        let profiles = known_profiles();
        assert!(
            profiles.len() >= 10,
            "expected at least 10 known profiles, got {}",
            profiles.len()
        );
    }

    #[test]
    fn known_profiles_unique_ids() {
        let profiles = known_profiles();
        let mut ids: Vec<_> = profiles.iter().map(|p| &p.id).collect();
        let original_len = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "profile IDs must be unique");
    }

    #[test]
    fn known_profiles_all_have_upstream_url() {
        for profile in known_profiles() {
            assert!(
                !profile.upstream_url.is_empty(),
                "profile {} missing upstream_url",
                profile.id
            );
        }
    }

    #[test]
    fn known_profiles_all_have_name() {
        for profile in known_profiles() {
            assert!(
                !profile.name.is_empty(),
                "profile {} missing name",
                profile.id
            );
        }
    }

    #[test]
    fn known_profiles_all_have_control_count() {
        for profile in known_profiles() {
            assert!(
                profile.control_count > 0,
                "profile {} has zero controls",
                profile.id
            );
        }
    }

    #[test]
    fn known_profiles_all_have_applicable_providers() {
        for profile in known_profiles() {
            assert!(
                !profile.applicable_providers.is_empty(),
                "profile {} has no applicable providers",
                profile.id
            );
        }
    }

    #[test]
    fn known_profiles_all_have_nist_families() {
        for profile in known_profiles() {
            assert!(
                !profile.nist_families.is_empty(),
                "profile {} has no NIST families",
                profile.id
            );
        }
    }

    #[test]
    fn known_profiles_contains_mitre_rhel9() {
        let profiles = known_profiles();
        assert!(
            profiles.iter().any(|p| p.id == "mitre-rhel9-stig"),
            "missing mitre-rhel9-stig"
        );
    }

    #[test]
    fn known_profiles_contains_dev_sec_linux() {
        let profiles = known_profiles();
        assert!(
            profiles.iter().any(|p| p.id == "dev-sec-linux-baseline"),
            "missing dev-sec-linux-baseline"
        );
    }

    #[test]
    fn known_profiles_contains_gcp_cis() {
        let profiles = known_profiles();
        assert!(
            profiles.iter().any(|p| p.id == "gcp-cis-benchmark"),
            "missing gcp-cis-benchmark"
        );
    }

    #[test]
    fn known_profiles_contains_k8s_stig() {
        let profiles = known_profiles();
        assert!(
            profiles.iter().any(|p| p.id == "mitre-k8s-cluster-stig"),
            "missing mitre-k8s-cluster-stig"
        );
    }

    #[test]
    fn known_profiles_contains_k3s_stig() {
        let profiles = known_profiles();
        assert!(
            profiles.iter().any(|p| p.id == "mitre-k3s-cluster-stig"),
            "missing mitre-k3s-cluster-stig"
        );
    }

    #[test]
    fn known_profiles_total_controls_above_1000() {
        let total: usize = known_profiles().iter().map(|p| p.control_count).sum();
        assert!(
            total > 1000,
            "expected total controls > 1000, got {total}"
        );
    }

    #[test]
    fn known_profiles_covers_multiple_sources() {
        let profiles = known_profiles();
        let has_mitre = profiles.iter().any(|p| p.source == ProfileSource::Mitre);
        let has_devsec = profiles.iter().any(|p| p.source == ProfileSource::DevSec);
        let has_gcp = profiles.iter().any(|p| p.source == ProfileSource::Gcp);
        assert!(has_mitre, "missing MITRE source");
        assert!(has_devsec, "missing DevSec source");
        assert!(has_gcp, "missing GCP source");
    }
}
