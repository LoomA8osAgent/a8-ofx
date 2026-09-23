//! Licence resolution for display and provenance — **never a load gate** (§22.4, §22.6).
//!
//! §22.4: "`src-tauri/src/ofx.rs` resolves each plugin's licence for display and
//! provenance, and records how it knew: (1) an identifier-prefix table we maintain
//! (`net.sf.openfx.` and `fr.inria.openfx.` → GPL-2.0-or-later); (2) a `LICENSE*` /
//! `COPYING*` file in the bundle's `Contents/Resources`; (3) the bundle `Info.plist`
//! `NSHumanReadableCopyright`; (4) `unknown`." That resolver is the host application's
//! (`ofx.rs`, in the `visualeyes` repo, outside this crate); this module holds the
//! identifier-prefix table and the resolution order both `visualeyes`'s `ofx.rs` and
//! VEtron consult, so the two hosts never disagree about a plugin's licence.

/// How the host determined a plugin's licence, in the resolution order of §22.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenceSource {
    /// Matched by `plugin_identifier` prefix against [`KNOWN_GPL_PREFIXES`].
    IdentifierPrefix,
    /// A `LICENSE*` / `COPYING*` file was found in `Contents/Resources`.
    BundleFile,
    /// Read from the bundle's `Info.plist` `NSHumanReadableCopyright`.
    BundleCopyright,
    /// None of the above resolved a licence.
    Unknown,
}

/// A resolved licence, with the source that produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLicence {
    pub spdx_or_label: String,
    pub source: LicenceSource,
}

/// `plugin_identifier` prefixes known, as of 2026-09-23, to belong to GPL-2.0-or-later
/// plugin sets (§22.2, §22.4: "82 and 20 `kPluginIdentifier` strings under those prefixes
/// in the openfx-misc + openfx-io clones"). This table never gates loading (§22.3) — it
/// only improves what the licence display shows before falling through to the bundle
/// file or copyright string.
pub const KNOWN_GPL_PREFIXES: &[(&str, &str)] = &[
    ("net.sf.openfx.", "GPL-2.0-or-later"),
    ("fr.inria.openfx.", "GPL-2.0-or-later"),
];

/// Resolve a licence label from a `plugin_identifier` alone, per the first step of
/// §22.4's order. Returns `None` if no known prefix matches — the caller then tries the
/// bundle-file and copyright-string steps (host-application filesystem access, outside
/// this crate).
pub fn resolve_by_identifier_prefix(plugin_identifier: &str) -> Option<ResolvedLicence> {
    KNOWN_GPL_PREFIXES
        .iter()
        .find(|(prefix, _)| plugin_identifier.starts_with(prefix))
        .map(|(_, label)| ResolvedLicence {
            spdx_or_label: (*label).to_string(),
            source: LicenceSource::IdentifierPrefix,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openfx_misc_identifiers_resolve_gpl() {
        let resolved = resolve_by_identifier_prefix("net.sf.openfx.GradePlugin").unwrap();
        assert_eq!(resolved.spdx_or_label, "GPL-2.0-or-later");
        assert_eq!(resolved.source, LicenceSource::IdentifierPrefix);
    }

    #[test]
    fn unknown_identifiers_fall_through() {
        assert!(resolve_by_identifier_prefix("com.example.SomePlugin").is_none());
    }
}
