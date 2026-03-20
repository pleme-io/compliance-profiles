use std::path::Path;
use std::process::Command;

use crate::error::{Error, Result};

/// Sync a profile from its upstream git repository as a git submodule.
///
/// # Errors
///
/// Returns an error if the git command fails or the path is invalid.
pub fn add_submodule(upstream_url: &str, target_path: &Path) -> Result<()> {
    let target = target_path
        .to_str()
        .ok_or_else(|| Error::InvalidProfile("target path is not valid UTF-8".to_string()))?;

    let output = Command::new("git")
        .args(["submodule", "add", upstream_url, target])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::InvalidProfile(format!(
            "git submodule add failed: {stderr}"
        )));
    }

    Ok(())
}

/// Update all git submodules to their tracked refs.
///
/// # Errors
///
/// Returns an error if the git command fails.
pub fn update_submodules() -> Result<()> {
    let output = Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::InvalidProfile(format!(
            "git submodule update failed: {stderr}"
        )));
    }

    Ok(())
}

/// Check if a path exists as a git submodule.
///
/// # Errors
///
/// Returns an error if the git command fails.
pub fn is_submodule(path: &Path) -> Result<bool> {
    let target = path
        .to_str()
        .ok_or_else(|| Error::InvalidProfile("path is not valid UTF-8".to_string()))?;

    let output = Command::new("git")
        .args(["submodule", "status", target])
        .output()?;

    Ok(output.status.success() && !output.stdout.is_empty())
}

/// Remove a git submodule at the given path.
///
/// # Errors
///
/// Returns an error if the git command fails.
pub fn remove_submodule(path: &Path) -> Result<()> {
    let target = path
        .to_str()
        .ok_or_else(|| Error::InvalidProfile("path is not valid UTF-8".to_string()))?;

    let output = Command::new("git")
        .args(["submodule", "deinit", "-f", target])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::InvalidProfile(format!(
            "git submodule deinit failed: {stderr}"
        )));
    }

    let output = Command::new("git")
        .args(["rm", "-f", target])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::InvalidProfile(format!("git rm failed: {stderr}")));
    }

    Ok(())
}
