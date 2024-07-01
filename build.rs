use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=openal-soft");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "bundled")]
    build_cmake();
}

#[cfg(feature = "bundled")]
macro_rules! bool {
    ($e:expr) => {
        if $e {
            "TRUE"
        } else {
            "FALSE"
        }
    };
}

#[cfg(feature = "bundled")]
fn build_cmake() {
    use cmake::Config;
    let dst = Config::new("openal-soft")
        .define("ALSOFT_UTILS", "OFF")
        .define("ALSOFT_NO_CONFIG_UTIL", "OFF")
        .define("ALSOFT_EXAMPLES", "OFF")
        .define("ALSOFT_TESTS", "OFF")
        .define("ALSOFT_INSTALL", "ON")
        .define("ALSOFT_INSTALL_CONFIG", "OFF")
        .define("ALSOFT_INSTALL_HRTF_DATA", "OFF")
        .define("ALSOFT_INSTALL_AMBDEC_PRESETS", "OFF")
        .define("ALSOFT_INSTALL_EXAMPLES", "OFF")
        .define("ALSOFT_INSTALL_UTILS", "OFF")
        .define("ALSOFT_UPDATE_BUILD_VERSION", "OFF")
        .define("ALSOFT_EAX", bool!(cfg!(feature = "enable_legacy_eax")))
        .define("ALSOFT_SEARCH_INSTALL_DATADIR", "OFF")
        .define("FORCE_STATIC_VCRT", bool!(cfg!(feature = "static_crt")))
        .define("ALSOFT_STATIC_LIBGCC", bool!(cfg!(feature = "static_crt")))
        .define(
            "ALSOFT_STATIC_LIBSTDCXX",
            bool!(cfg!(feature = "static_cxxrt")),
        )
        .define(
            "ALSOFT_STATIC_WINPTHREAD",
            bool!(cfg!(feature = "static_cxxrt")),
        )
        .define(
            "LIBTYPE",
            if cfg!(feature = "static_link") {
                "STATIC"
            } else {
                "SHARED"
            },
        )
        .define("ALSOFT_CPUEXT_SSE", bool!(cfg!(feature = "enable_sse")))
        .define("ALSOFT_CPUEXT_SSE2", bool!(cfg!(feature = "enable_sse2")))
        .define("ALSOFT_CPUEXT_SSE3", bool!(cfg!(feature = "enable_sse3")))
        .define(
            "ALSOFT_CPUEXT_SSE4_1",
            bool!(cfg!(feature = "enable_sse4_1")),
        )
        .define(
            "ALSOFT_CPUEXT_NEON",
            bool!(cfg!(feature = "enable_arm_neon")),
        )
        .define(
            "ALSOFT_ENABLE_SSE2_CODEGEN",
            bool!(cfg!(feature = "enable_sse2_codegen")),
        )
        .define("ALSOFT_RTKIT", bool!(cfg!(feature = "enable_rtkit")))
        .define(
            "ALSOFT_BACKEND_PIPEWIRE",
            bool!(cfg!(feature = "enable_pipewire")),
        )
        .define(
            "ALSOFT_BACKEND_PULSEAUDIO",
            bool!(cfg!(feature = "enable_pulseaudio")),
        )
        .define("ALSOFT_BACKEND_ALSA", bool!(cfg!(feature = "enable_alsa")))
        .define("ALSOFT_BACKEND_OSS", bool!(cfg!(feature = "enable_oss")))
        .define(
            "ALSOFT_BACKEND_SOLARIS",
            bool!(cfg!(feature = "enable_solaris")),
        )
        .define(
            "ALSOFT_BACKEND_SNDIO",
            bool!(cfg!(feature = "enable_sndio")),
        )
        .define(
            "ALSOFT_BACKEND_WINMM",
            bool!(cfg!(feature = "enable_winmm")),
        )
        .define(
            "ALSOFT_BACKEND_DSOUND",
            bool!(cfg!(feature = "enable_directsound")),
        )
        .define(
            "ALSOFT_BACKEND_WASAPI",
            bool!(cfg!(feature = "enable_wasapi")),
        )
        .define("ALSOFT_BACKEND_JACK", bool!(cfg!(feature = "enable_jack")))
        .define(
            "ALSOFT_BACKEND_COREAUDIO",
            bool!(cfg!(feature = "enable_coreaudio")),
        )
        .define("ALSOFT_BACKEND_OBOE", bool!(cfg!(feature = "enable_oboe")))
        .define(
            "ALSOFT_BACKEND_OPENSL",
            bool!(cfg!(feature = "enable_opensl")),
        )
        .define(
            "ALSOFT_BACKEND_PORTAUDIO",
            bool!(cfg!(feature = "enable_portaudio")),
        )
        .define("ALSOFT_BACKEND_SDL2", bool!(cfg!(feature = "enable_sdl2")))
        .define(
            "ALSOFT_BACKEND_WAVE",
            bool!(cfg!(feature = "enable_wave_writer")),
        )
        .define("ALSOFT_REQUIRE_SSE", bool!(cfg!(feature = "require_sse")))
        .define("ALSOFT_REQUIRE_SSE2", bool!(cfg!(feature = "require_sse2")))
        .define("ALSOFT_REQUIRE_SSE3", bool!(cfg!(feature = "require_sse3")))
        .define(
            "ALSOFT_REQUIRE_SSE4_1",
            bool!(cfg!(feature = "require_sse4_1")),
        )
        .define(
            "ALSOFT_REQUIRE_NEON",
            bool!(cfg!(feature = "require_arm_neon")),
        )
        .define(
            "ALSOFT_REQUIRE_RTKIT",
            bool!(cfg!(feature = "require_rtkit")),
        )
        .define(
            "ALSOFT_REQUIRE_PIPEWIRE",
            bool!(cfg!(feature = "require_pipewire")),
        )
        .define(
            "ALSOFT_REQUIRE_PULSEAUDIO",
            bool!(cfg!(feature = "require_pulseaudio")),
        )
        .define("ALSOFT_REQUIRE_ALSA", bool!(cfg!(feature = "require_alsa")))
        .define("ALSOFT_REQUIRE_OSS", bool!(cfg!(feature = "require_oss")))
        .define(
            "ALSOFT_REQUIRE_SOLARIS",
            bool!(cfg!(feature = "require_solaris")),
        )
        .define(
            "ALSOFT_REQUIRE_SNDIO",
            bool!(cfg!(feature = "require_sndio")),
        )
        .define(
            "ALSOFT_REQUIRE_WINMM",
            bool!(cfg!(feature = "require_winmm")),
        )
        .define(
            "ALSOFT_REQUIRE_DSOUND",
            bool!(cfg!(feature = "require_directsound")),
        )
        .define(
            "ALSOFT_REQUIRE_WASAPI",
            bool!(cfg!(feature = "require_wasapi")),
        )
        .define("ALSOFT_REQUIRE_JACK", bool!(cfg!(feature = "require_jack")))
        .define(
            "ALSOFT_REQUIRE_COREAUDIO",
            bool!(cfg!(feature = "require_coreaudio")),
        )
        .define("ALSOFT_REQUIRE_OBOE", bool!(cfg!(feature = "require_oboe")))
        .define(
            "ALSOFT_REQUIRE_OPENSL",
            bool!(cfg!(feature = "require_opensl")),
        )
        .define(
            "ALSOFT_REQUIRE_PORTAUDIO",
            bool!(cfg!(feature = "require_portaudio")),
        )
        .define("ALSOFT_REQUIRE_SDL2", bool!(cfg!(feature = "require_sdl2")))
        .define(
            "ALSOFT_REQUIRE_WAVE",
            bool!(cfg!(feature = "require_wave_writer")),
        )
        .build();

    let lib = dst.join("lib");
    let bin = dst.join("bin");

    println!("cargo:rustc-link-lib=OpenAL32");
    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-search={}", lib.display());

    copy_binaries(&bin, &find_cargo_target_dir());
}

fn copy_binaries(dst: &Path, src: &Path) {
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, dst.join(path.file_name().unwrap())).unwrap();
        }
    }
}

fn find_cargo_target_dir() -> PathBuf {
    // Infer the top level cargo target dir from the OUT_DIR by searching
    // upwards until we get to $CARGO_TARGET_DIR/build/ (which is always one
    // level up from the deepest directory containing our package name)
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let mut out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    loop {
        {
            let final_path_segment = out_dir.file_name().unwrap();
            if final_path_segment.to_string_lossy().contains(&pkg_name) {
                break;
            }
        }
        if !out_dir.pop() {
            panic!("Malformed build path: {}", out_dir.to_string_lossy());
        }
    }
    out_dir.pop();
    out_dir.pop();
    out_dir
}
