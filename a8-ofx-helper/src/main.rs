//! `a8-ofx-helper` — the out-of-process runner that loads and executes OFX plugins in a
//! process separate from the host application.
//!
//! §22.4 "Process (D-15, default: helper)": "Plugins load in `a8-ofx-host`, our own small
//! helper binary shipped inside `visualeyes.app` and started at first OFX use — not in the
//! visualeyes process. A third-party plugin that crashes then kills one effect, not the
//! live output... Frames cross the process boundary as **IOSurfaces**... wrapped as Metal
//! textures (`objc2-io-surface`, `metal`); parameters and render calls go over a pipe."
//! (The binary is named `a8-ofx-helper` in this repo; the spec text names it inline as
//! `a8-ofx-host` in one place — the two-crate split, host library + helper binary, is
//! this repo's structure.)
//!
//! VEtron links `a8-ofx-host` in-process instead of spawning this helper (§22.4: "VEtron
//! links `a8-ofx` in-process (it is a compositor, not a live-performance surface; D-15
//! does not bind it)").

use a8_ofx_host::discovery;

fn main() {
    // §22.4 discovery: search OFX_PLUGIN_PATH + /Library/OFX/Plugins + any extra folders
    // the host application passes on the command line (its Settings library-folders
    // list, resolved by the caller, not this binary).
    let extra_dirs: Vec<std::path::PathBuf> = std::env::args().skip(1).map(Into::into).collect();

    let dirs = discovery::search_dirs(&extra_dirs);
    let bundles = discovery::scan(&dirs);

    eprintln!("a8-ofx-helper: {} plugin bundle(s) found", bundles.len());
    for bundle in &bundles {
        eprintln!("  {}", bundle.bundle_path.display());
    }

    // The parameter/render pipe protocol (§22.4: "parameters and render calls go over a
    // pipe") and the IOSurface frame handoff (§22.4, §22.5 Stage B) are not implemented
    // in this scaffold — they are the next lane (roadmap/natron-rust-arc.md §The OFX
    // host, lanes O0-O9).
}
