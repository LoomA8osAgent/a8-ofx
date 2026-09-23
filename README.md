# a8-ofx

An [OpenFX](https://openfxassociation.org/) plugin host, written in Rust, shared by
[visualeyes](https://github.com/LoomA8osAgent) and VEtron.

## Licence

Dual-licensed under **MIT OR Apache-2.0** (your choice). See `LICENSE-MIT` and
`LICENSE-APACHE`.

This crate is a **clean-room implementation** of an OpenFX host. It is written from the
[OpenFX specification](https://openfx.readthedocs.io/) and the Academy Software
Foundation's BSD-3-Clause headers and HostSupport / Examples code
(https://github.com/AcademySoftwareFoundation/openfx) — never from any GPL-licensed OFX
host (Natron, DaVinci Resolve, or any fork of either). No GPL source has been read,
copied, translated, or otherwise consulted in the writing of this crate.

`a8-ofx` **loads** whatever OFX plugins the user has installed on their own machine —
including GPL-licensed plugin sets such as `openfx-misc` and `openfx-io` — but it
**ships none of them**. The host itself, and everything in this repository, is
permissively licensed and distributable inside closed-source applications.

## Crates

- **`a8-ofx-host`** — the library: plugin discovery in the standard OFX search paths, and
  the host-side C ABI surface (property, image effect, parameter, memory, multithread,
  message, progress, and timeline suites).
- **`a8-ofx-helper`** — the out-of-process binary that loads and runs plugins in a
  separate process from the host application, so a crashing third-party plugin cannot
  take down the host.

## Status

Early scaffold. See each crate's source for `// spec §…` comments pointing at the design
document this host implements (`visualeyes` repo, `specs/editors/compositor-natron.md`
§22, "OFX effects in visualeyes — `a8-ofx`, our own host").
