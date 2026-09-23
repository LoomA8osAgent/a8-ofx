//! `a8-ofx-host` — a clean-room [OpenFX](https://openfx.readthedocs.io/) host library.
//!
//! Written from the OpenFX specification and the Academy Software Foundation's
//! BSD-3-Clause headers, HostSupport and Examples code
//! (<https://github.com/AcademySoftwareFoundation/openfx>). Never written from, or with
//! reference to, any GPL-licensed OFX host (Natron or any fork of it, DaVinci Resolve).
//!
//! Design document: `visualeyes` repo, `specs/editors/compositor-natron.md` §22 ("OFX
//! effects in visualeyes — `a8-ofx`, our own host"). Section references below (`§22.N`)
//! point into that document.

pub mod abi;
pub mod discovery;
pub mod licence;
