//! The host-side C ABI surface: the plugin entry point, the suites, and the property /
//! image-effect / parameter / memory / multithread / message / progress / timeline
//! constants defined by the OpenFX specification.
//!
//! §22.4 "Suites, v1": property, image effect, parameter, memory, multithread, message
//! (v1 + v2), progress, timeline. Pixel depths 8-bit, 16-bit half and 32-bit float RGBA.
//! GPU stage B adds the OFX 1.5 Metal render properties
//! (`kOfxImageEffectPropMetalRenderSupported`, `kOfxImageEffectPropMetalEnabled`,
//! `kOfxImageEffectPropMetalCommandQueue`, per
//! <https://raw.githubusercontent.com/AcademySoftwareFoundation/openfx/main/include/ofxGPURender.h>).
//! The OpenGL render suite is not offered in v1; the interact suite (viewer overlays) is
//! not offered in v1 (§22.4).
//!
//! Symbol and constant names below are drawn from the public OFX C headers (BSD-3-Clause,
//! Academy Software Foundation, <https://github.com/AcademySoftwareFoundation/openfx>) —
//! a published API, not host implementation code. The bodies here are `a8-ofx`'s own,
//! written against that API and the specification text, never against any GPL host's
//! source.

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

/// `OfxStatus` — the C return code every suite function and plugin entry point uses.
/// Values per the OFX Core API (`ofxCore.h`).
pub type OfxStatus = c_int;

pub const OFX_STATUS_OK: OfxStatus = 0;
pub const OFX_STATUS_FAILED: OfxStatus = 1;
pub const OFX_STATUS_ERR_FATAL: OfxStatus = 2;
pub const OFX_STATUS_ERR_UNKNOWN: OfxStatus = 3;
pub const OFX_STATUS_ERR_MISSING_HOST_FEATURE: OfxStatus = 4;
pub const OFX_STATUS_ERR_UNSUPPORTED: OfxStatus = 5;
pub const OFX_STATUS_ERR_EXISTS: OfxStatus = 6;
pub const OFX_STATUS_ERR_FORMAT: OfxStatus = 7;
pub const OFX_STATUS_ERR_MEMORY: OfxStatus = 8;
pub const OFX_STATUS_ERR_BAD_HANDLE: OfxStatus = 9;
pub const OFX_STATUS_ERR_BAD_INDEX: OfxStatus = 10;
pub const OFX_STATUS_ERR_VALUE: OfxStatus = 11;
pub const OFX_STATUS_REPLY_YES: OfxStatus = 12;
pub const OFX_STATUS_REPLY_NO: OfxStatus = 13;
pub const OFX_STATUS_REPLY_DEFAULT: OfxStatus = 14;

/// An opaque handle, matching every `Ofx*Handle` typedef in the C headers (each is a
/// distinct pointer type at the API level; the host never dereferences these itself).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpaqueHandle(pub *mut c_void);

/// A named, versioned suite fetched via `OfxHost::fetchSuite`. §22.4 v1 offers the suites
/// named in [`SuiteName`]; GPU render properties (stage B) ride the existing image-effect
/// suite as additional properties, not a new suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuiteName {
    Property,
    ImageEffect,
    Parameter,
    Memory,
    MultiThread,
    Message,
    MessageV2,
    Progress,
    Timeline,
}

impl SuiteName {
    /// The `suiteName` string a plugin passes to `fetchSuite`, per `ofxCore.h` /
    /// `ofxImageEffect.h` / `ofxParam.h` / `ofxMemory.h` / `ofxMultiThread.h` /
    /// `ofxMessage.h` / `ofxProgress.h` / `ofxKeySyms.h` (timeline lives in
    /// `ofxImageEffect.h` as `kOfxImageEffectSuite`'s timeline calls in OFX 1.2+).
    pub const fn c_name(self) -> &'static str {
        match self {
            SuiteName::Property => "OfxPropertySuite",
            SuiteName::ImageEffect => "OfxImageEffectSuite",
            SuiteName::Parameter => "OfxParameterSuite",
            SuiteName::Memory => "OfxMemorySuite",
            SuiteName::MultiThread => "OfxMultiThreadSuite",
            SuiteName::Message => "OfxMessageSuite",
            SuiteName::MessageV2 => "OfxMessageSuiteV2",
            SuiteName::Progress => "OfxProgressSuite",
            SuiteName::Timeline => "OfxTimeLineSuite",
        }
    }
}

/// Pixel depths the host advertises support for, per §22.4: "8-bit, 16-bit half and
/// 32-bit float RGBA." Corresponds to `kOfxBitDepthByte`, `kOfxBitDepthShort` (used here
/// for the half-float case per plugin convention) and `kOfxBitDepthFloat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Byte8,
    Half16,
    Float32,
}

/// GPU render properties offered starting at stage B (§22.4), the OFX 1.5 Metal
/// extension. Names per
/// <https://raw.githubusercontent.com/AcademySoftwareFoundation/openfx/main/include/ofxGPURender.h>.
pub mod metal_props {
    pub const SUPPORTED: &str = "OfxImageEffectPropMetalRenderSupported";
    pub const ENABLED: &str = "OfxImageEffectPropMetalEnabled";
    pub const COMMAND_QUEUE: &str = "OfxImageEffectPropMetalCommandQueue";
}

/// The plugin entry point signature every `.ofx.bundle` binary exports as
/// `OfxGetNumberOfPlugins` + `OfxGetPlugin`. The host calls these via `dlopen`/`dlsym`
/// (macOS: `libloading`, dependency to be added when the loader lands — discovery.rs
/// resolves the bundle path first, per §22.4).
pub type OfxGetNumberOfPluginsFn = unsafe extern "C" fn() -> c_int;
pub type OfxGetPluginFn = unsafe extern "C" fn(nth: c_int) -> *const OfxPlugin;

/// Mirrors the C `OfxPlugin` struct (`ofxCore.h`): the four-field descriptor every
/// plugin's `OfxGetPlugin` returns.
#[repr(C)]
pub struct OfxPlugin {
    pub plugin_api: *const c_char,
    pub api_version: c_int,
    pub plugin_identifier: *const c_char,
    pub plugin_version_major: c_int,
    pub plugin_version_minor: c_int,
    pub set_host: extern "C" fn(host: *const OfxHost),
    pub main_entry: extern "C" fn(
        action: *const c_char,
        handle: *const c_void,
        in_args: *const c_void,
        out_args: *const c_void,
    ) -> OfxStatus,
}

/// Mirrors the C `OfxHost` struct (`ofxCore.h`): the host descriptor + suite-fetch
/// function every plugin receives via `set_host`.
#[repr(C)]
pub struct OfxHost {
    pub host: *const c_void,
    pub fetch_suite: extern "C" fn(
        host: *const c_void,
        suite_name: *const c_char,
        suite_version: c_int,
    ) -> *const c_void,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_names_match_the_ofx_headers() {
        assert_eq!(SuiteName::ImageEffect.c_name(), "OfxImageEffectSuite");
        assert_eq!(SuiteName::Timeline.c_name(), "OfxTimeLineSuite");
    }
}
