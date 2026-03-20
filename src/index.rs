use std::collections::BTreeSet;
use std::path::Path;

use chrono::Utc;

use crate::error::{Error, Result};
use crate::profile::{IndexStats, ProfileEntry};

/// The full profile index manifest.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ProfileIndex {
    /// Version of the index format.
    pub version: String,
    /// When the index was last updated.
    pub updated_at: chrono::DateTime<Utc>,
    /// All profile entries.
    pub profiles: Vec<ProfileEntry>,
    /// Summary statistics.
    pub stats: IndexStats,
}

impl ProfileIndex {
    /// Create a new, empty profile index.
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            updated_at: Utc::now(),
            profiles: Vec::new(),
            stats: IndexStats::empty(),
        }
    }

    /// Add a profile entry to the index.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateProfile` if a profile with the same ID already exists.
    pub fn add_profile(&mut self, entry: ProfileEntry) -> Result<()> {
        if self.profiles.iter().any(|p| p.id == entry.id) {
            return Err(Error::DuplicateProfile(entry.id));
        }
        self.profiles.push(entry);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a profile by ID, returning the removed entry if found.
    pub fn remove_profile(&mut self, id: &str) -> Option<ProfileEntry> {
        if let Some(pos) = self.profiles.iter().position(|p| p.id == id) {
            self.updated_at = Utc::now();
            Some(self.profiles.remove(pos))
        } else {
            None
        }
    }

    /// Find all profiles applicable to a given provider.
    #[must_use]
    pub fn find_by_provider(&self, provider: &str) -> Vec<&ProfileEntry> {
        self.profiles
            .iter()
            .filter(|p| {
                p.applicable_providers
                    .iter()
                    .any(|prov| prov.eq_ignore_ascii_case(provider))
            })
            .collect()
    }

    /// Find all profiles covering a given NIST 800-53 family.
    #[must_use]
    pub fn find_by_nist_family(&self, family: &str) -> Vec<&ProfileEntry> {
        self.profiles
            .iter()
            .filter(|p| {
                p.nist_families
                    .iter()
                    .any(|f| f.eq_ignore_ascii_case(family))
            })
            .collect()
    }

    /// Recompute summary statistics from the current profile list.
    pub fn compute_stats(&mut self) {
        let mut providers: BTreeSet<String> = BTreeSet::new();
        let mut nist_families: BTreeSet<String> = BTreeSet::new();

        let mut total_controls = 0;
        let mut total_rspec = 0;

        for profile in &self.profiles {
            total_controls += profile.control_count;
            if profile.rspec_generated {
                total_rspec += 1;
            }
            for provider in &profile.applicable_providers {
                providers.insert(provider.clone());
            }
            for family in &profile.nist_families {
                nist_families.insert(family.clone());
            }
        }

        self.stats = IndexStats {
            total_profiles: self.profiles.len(),
            total_controls,
            total_rspec_generated: total_rspec,
            providers_covered: providers.into_iter().collect(),
            nist_families_covered: nist_families.into_iter().collect(),
        };
    }

    /// Total number of controls across all profiles.
    #[must_use]
    pub fn total_controls(&self) -> usize {
        self.profiles.iter().map(|p| p.control_count).sum()
    }

    /// Save the index to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written or serialization fails.
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load an index from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or deserialization fails.
    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let index: Self = serde_json::from_str(&json)?;
        Ok(index)
    }

    /// Return the number of profiles in the index.
    #[must_use]
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// Check whether the index is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

impl Default for ProfileIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::ProfileSource;
    use tempfile::TempDir;

    fn make_entry(id: &str) -> ProfileEntry {
        ProfileEntry {
            id: id.to_string(),
            name: format!("Profile {id}"),
            source: ProfileSource::Custom,
            upstream_url: format!("https://github.com/example/{id}"),
            upstream_ref: "main".to_string(),
            control_count: 10,
            profile_hash: format!("hash_{id}"),
            applicable_providers: vec!["aws".to_string()],
            nist_families: vec!["AC".to_string()],
            cis_benchmarks: vec![],
            rspec_generated: false,
            last_synced: None,
        }
    }

    #[test]
    fn new_index_is_empty() {
        let index = ProfileIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        assert_eq!(index.version, "1.0.0");
    }

    #[test]
    fn add_profile_increases_count() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("test-1")).unwrap();
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn add_duplicate_profile_fails() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("dup")).unwrap();
        let result = index.add_profile(make_entry("dup"));
        assert!(result.is_err());
    }

    #[test]
    fn remove_existing_profile() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("removable")).unwrap();
        let removed = index.remove_profile("removable");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id, "removable");
        assert!(index.is_empty());
    }

    #[test]
    fn remove_nonexistent_profile() {
        let mut index = ProfileIndex::new();
        let removed = index.remove_profile("ghost");
        assert!(removed.is_none());
    }

    #[test]
    fn find_by_provider_matches() {
        let mut index = ProfileIndex::new();
        let mut entry = make_entry("aws-profile");
        entry.applicable_providers = vec!["aws".to_string(), "gcp".to_string()];
        index.add_profile(entry).unwrap();
        index.add_profile(make_entry("other")).unwrap();

        let aws = index.find_by_provider("aws");
        assert_eq!(aws.len(), 2); // both have "aws"
    }

    #[test]
    fn find_by_provider_case_insensitive() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("test")).unwrap();
        let results = index.find_by_provider("AWS");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn find_by_provider_no_match() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("test")).unwrap();
        let results = index.find_by_provider("azure");
        assert!(results.is_empty());
    }

    #[test]
    fn find_by_nist_family_matches() {
        let mut index = ProfileIndex::new();
        let mut entry = make_entry("nist-test");
        entry.nist_families = vec!["AC".to_string(), "CM".to_string(), "SC".to_string()];
        index.add_profile(entry).unwrap();

        let results = index.find_by_nist_family("CM");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "nist-test");
    }

    #[test]
    fn find_by_nist_family_case_insensitive() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("test")).unwrap();
        let results = index.find_by_nist_family("ac");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn find_by_nist_family_no_match() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("test")).unwrap();
        let results = index.find_by_nist_family("AU");
        assert!(results.is_empty());
    }

    #[test]
    fn compute_stats_empty_index() {
        let mut index = ProfileIndex::new();
        index.compute_stats();
        assert_eq!(index.stats.total_profiles, 0);
        assert_eq!(index.stats.total_controls, 0);
        assert_eq!(index.stats.total_rspec_generated, 0);
    }

    #[test]
    fn compute_stats_with_profiles() {
        let mut index = ProfileIndex::new();

        let mut e1 = make_entry("p1");
        e1.control_count = 50;
        e1.applicable_providers = vec!["aws".to_string()];
        e1.nist_families = vec!["AC".to_string(), "CM".to_string()];
        e1.rspec_generated = true;

        let mut e2 = make_entry("p2");
        e2.control_count = 30;
        e2.applicable_providers = vec!["gcp".to_string()];
        e2.nist_families = vec!["CM".to_string(), "SC".to_string()];

        index.add_profile(e1).unwrap();
        index.add_profile(e2).unwrap();
        index.compute_stats();

        assert_eq!(index.stats.total_profiles, 2);
        assert_eq!(index.stats.total_controls, 80);
        assert_eq!(index.stats.total_rspec_generated, 1);
        assert_eq!(index.stats.providers_covered, vec!["aws", "gcp"]);
        assert_eq!(
            index.stats.nist_families_covered,
            vec!["AC", "CM", "SC"]
        );
    }

    #[test]
    fn total_controls_sums_correctly() {
        let mut index = ProfileIndex::new();
        let mut e1 = make_entry("a");
        e1.control_count = 25;
        let mut e2 = make_entry("b");
        e2.control_count = 75;
        index.add_profile(e1).unwrap();
        index.add_profile(e2).unwrap();
        assert_eq!(index.total_controls(), 100);
    }

    #[test]
    fn total_controls_empty_index() {
        let index = ProfileIndex::new();
        assert_eq!(index.total_controls(), 0);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("index.json");

        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("saved")).unwrap();
        index.compute_stats();
        index.save(&path).unwrap();

        let loaded = ProfileIndex::load(&path).unwrap();
        assert_eq!(loaded.profiles.len(), 1);
        assert_eq!(loaded.profiles[0].id, "saved");
        assert_eq!(loaded.version, "1.0.0");
    }

    #[test]
    fn load_nonexistent_file_fails() {
        let result = ProfileIndex::load(Path::new("/nonexistent/index.json"));
        assert!(result.is_err());
    }

    #[test]
    fn serde_roundtrip() {
        let mut index = ProfileIndex::new();
        index.add_profile(make_entry("serde-test")).unwrap();
        index.compute_stats();

        let json = serde_json::to_string(&index).unwrap();
        let deserialized: ProfileIndex = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.profiles.len(), 1);
        assert_eq!(deserialized.profiles[0].id, "serde-test");
    }

    #[test]
    fn default_is_new() {
        let index = ProfileIndex::default();
        assert!(index.is_empty());
        assert_eq!(index.version, "1.0.0");
    }

    #[test]
    fn multiple_profiles_same_provider() {
        let mut index = ProfileIndex::new();
        let mut e1 = make_entry("p1");
        e1.applicable_providers = vec!["kubernetes".to_string()];
        let mut e2 = make_entry("p2");
        e2.applicable_providers = vec!["kubernetes".to_string()];
        index.add_profile(e1).unwrap();
        index.add_profile(e2).unwrap();

        let results = index.find_by_provider("kubernetes");
        assert_eq!(results.len(), 2);
    }
}
