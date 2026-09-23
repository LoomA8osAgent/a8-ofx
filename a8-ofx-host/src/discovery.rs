//! Plugin discovery in the standard OFX search paths.
//!
//! §22.4: "On macOS, OFX searches 'the \';\'-separated directory list specified by the
//! environment variable OFX_PLUGIN_PATH' then 'the directory `/Library/OFX/Plugins`'"
//! (<https://openfx.readthedocs.io/en/main/Reference/ofxPackaging.html>), plus folders the
//! user adds in Settings (disk, `/api/settings` on the visualeyes side — outside this
//! crate's scope). Bundles are `NAME.ofx.bundle/Contents/MacOS/NAME.ofx`, universal
//! binaries (same page). The scan result is cached on disk, keyed by bundle path + mtime,
//! with a rescan action (§22.4) — the cache is host-application state, not this crate's.

use std::env;
use std::path::PathBuf;

/// The fixed, non-negotiable search location on macOS, per the OFX packaging reference.
pub const MACOS_SYSTEM_PLUGIN_DIR: &str = "/Library/OFX/Plugins";

/// A single located `.ofx.bundle`, before it has been loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleCandidate {
    /// Path to the `NAME.ofx.bundle` directory.
    pub bundle_path: PathBuf,
    /// Path to the executable inside `Contents/MacOS/`.
    pub binary_path: PathBuf,
}

/// The ordered list of directories OFX searches, per the specification: `OFX_PLUGIN_PATH`
/// entries first, then the platform system directory. `extra_dirs` are folders the host
/// application adds (visualeyes: the Settings library-folders list, §22.4).
pub fn search_dirs(extra_dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(path_var) = env::var("OFX_PLUGIN_PATH") {
        for entry in env::split_paths(&path_var) {
            dirs.push(entry);
        }
    }

    dirs.push(PathBuf::from(MACOS_SYSTEM_PLUGIN_DIR));
    dirs.extend(extra_dirs.iter().cloned());
    dirs
}

/// Walk `dirs` for `*.ofx.bundle` directories and resolve each to its binary under
/// `Contents/MacOS/`. Bundles are universal Mach-O binaries; architecture selection is the
/// operating system's (`dlopen`), not this function's.
///
/// A directory that does not exist, or is not readable, is skipped — a missing search
/// path is not an error (the standard system directory rarely exists until a plugin is
/// installed).
pub fn scan(dirs: &[PathBuf]) -> Vec<BundleCandidate> {
    let mut found = Vec::new();

    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let is_bundle = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("bundle"))
                .unwrap_or(false)
                && path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|stem| stem.ends_with(".ofx"))
                    .unwrap_or(false);

            if !is_bundle {
                continue;
            }

            let Some(name) = path
                .file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.strip_suffix(".ofx"))
            else {
                continue;
            };

            let binary_path = path.join("Contents/MacOS").join(format!("{name}.ofx"));
            if binary_path.exists() {
                found.push(BundleCandidate {
                    bundle_path: path,
                    binary_path,
                });
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_dirs_always_includes_the_system_dir() {
        let dirs = search_dirs(&[]);
        assert!(dirs.iter().any(|d| d == &PathBuf::from(MACOS_SYSTEM_PLUGIN_DIR)));
    }

    #[test]
    fn search_dirs_appends_extra_dirs_after_system_dir() {
        let extra = PathBuf::from("/tmp/some-extra-ofx-dir");
        let dirs = search_dirs(&[extra.clone()]);
        assert_eq!(dirs.last(), Some(&extra));
    }

    #[test]
    fn scan_of_a_nonexistent_dir_is_empty_not_an_error() {
        let dirs = vec![PathBuf::from("/this/path/does/not/exist/a8-ofx-test")];
        assert!(scan(&dirs).is_empty());
    }
}
