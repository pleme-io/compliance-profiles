use std::path::Path;

use crate::error::Result;

/// Compute BLAKE3 hash of all files in a profile directory.
///
/// Walks the directory recursively, sorts files by path for determinism,
/// hashes each file, then combines all individual hashes into a final hash.
///
/// # Errors
///
/// Returns an error if the directory cannot be read or any file cannot be hashed.
pub fn hash_profile_directory(path: &Path) -> Result<String> {
    let mut file_paths: Vec<_> = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();

    // Sort for deterministic ordering.
    file_paths.sort();

    let mut hasher = blake3::Hasher::new();

    for file_path in &file_paths {
        // Include the relative path in the hash so renames are detected.
        let relative = file_path
            .strip_prefix(path)
            .unwrap_or(file_path);
        hasher.update(relative.to_string_lossy().as_bytes());

        let contents = std::fs::read(file_path)?;
        hasher.update(&contents);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

/// Compute BLAKE3 hash of a single InSpec control file.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn hash_control_file(path: &Path) -> Result<String> {
    let contents = std::fs::read(path)?;
    let hash = blake3::hash(&contents);
    Ok(hash.to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn hash_single_control_file() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.rb");
        fs::write(&file_path, "control 'test-01' do\n  impact 1.0\nend\n").unwrap();

        let hash = hash_control_file(&file_path).unwrap();
        assert_eq!(hash.len(), 64); // BLAKE3 hex is 64 chars
    }

    #[test]
    fn hash_control_file_deterministic() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("test.rb");
        fs::write(&file_path, "control 'test-01' do\n  impact 1.0\nend\n").unwrap();

        let hash1 = hash_control_file(&file_path).unwrap();
        let hash2 = hash_control_file(&file_path).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn hash_different_files_differ() {
        let dir = TempDir::new().unwrap();
        let file_a = dir.path().join("a.rb");
        let file_b = dir.path().join("b.rb");
        fs::write(&file_a, "control 'a' do\nend\n").unwrap();
        fs::write(&file_b, "control 'b' do\nend\n").unwrap();

        let hash_a = hash_control_file(&file_a).unwrap();
        let hash_b = hash_control_file(&file_b).unwrap();
        assert_ne!(hash_a, hash_b);
    }

    #[test]
    fn hash_profile_directory_basic() {
        let dir = TempDir::new().unwrap();
        let controls = dir.path().join("controls");
        fs::create_dir_all(&controls).unwrap();
        fs::write(controls.join("os_01.rb"), "control 'os-01' do\nend\n").unwrap();
        fs::write(controls.join("os_02.rb"), "control 'os-02' do\nend\n").unwrap();
        fs::write(dir.path().join("inspec.yml"), "name: test\n").unwrap();

        let hash = hash_profile_directory(dir.path()).unwrap();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn hash_profile_directory_deterministic() {
        let dir = TempDir::new().unwrap();
        let controls = dir.path().join("controls");
        fs::create_dir_all(&controls).unwrap();
        fs::write(controls.join("os_01.rb"), "control 'os-01' do\nend\n").unwrap();
        fs::write(dir.path().join("inspec.yml"), "name: test\n").unwrap();

        let hash1 = hash_profile_directory(dir.path()).unwrap();
        let hash2 = hash_profile_directory(dir.path()).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn hash_profile_directory_different_content_differs() {
        let dir1 = TempDir::new().unwrap();
        fs::write(dir1.path().join("test.rb"), "content A").unwrap();

        let dir2 = TempDir::new().unwrap();
        fs::write(dir2.path().join("test.rb"), "content B").unwrap();

        let hash1 = hash_profile_directory(dir1.path()).unwrap();
        let hash2 = hash_profile_directory(dir2.path()).unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn hash_empty_directory() {
        let dir = TempDir::new().unwrap();

        let hash = hash_profile_directory(dir.path()).unwrap();
        // Empty directory still produces a valid hash (the initial state of the hasher).
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn hash_profile_directory_order_independent_of_creation() {
        // Create two directories with same files but added in different order.
        let dir1 = TempDir::new().unwrap();
        fs::write(dir1.path().join("a.rb"), "aaa").unwrap();
        fs::write(dir1.path().join("b.rb"), "bbb").unwrap();

        let dir2 = TempDir::new().unwrap();
        fs::write(dir2.path().join("b.rb"), "bbb").unwrap();
        fs::write(dir2.path().join("a.rb"), "aaa").unwrap();

        let hash1 = hash_profile_directory(dir1.path()).unwrap();
        let hash2 = hash_profile_directory(dir2.path()).unwrap();
        assert_eq!(hash1, hash2, "hash should be independent of file creation order");
    }

    #[test]
    fn hash_profile_directory_rename_changes_hash() {
        let dir1 = TempDir::new().unwrap();
        fs::write(dir1.path().join("old_name.rb"), "content").unwrap();

        let dir2 = TempDir::new().unwrap();
        fs::write(dir2.path().join("new_name.rb"), "content").unwrap();

        let hash1 = hash_profile_directory(dir1.path()).unwrap();
        let hash2 = hash_profile_directory(dir2.path()).unwrap();
        assert_ne!(hash1, hash2, "renaming a file should change the directory hash");
    }

    #[test]
    fn hash_control_file_nonexistent_fails() {
        let result = hash_control_file(Path::new("/nonexistent/file.rb"));
        assert!(result.is_err());
    }
}
