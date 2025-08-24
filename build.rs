#[cfg(feature = "build")]
mod cmake_build {
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, io};
    macro_rules! feature_cmake_str {
        ($feature:literal) => {
            if cfg!(feature = $feature) {
                "ON"
            } else {
                "OFF"
            }
        };
    }
    pub fn out_dir() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").unwrap())
    }

    pub fn try_clone_repo() -> io::Result<()> {
        const REPO_URL: &str = "https://github.com/kcat/openal-soft.git";

        let path = out_dir().join("openal-soft");

        if path.exists() {
            Command::new("git")
                .current_dir(&path)
                .arg("fetch")
                .arg("--all")
                .status()?;

            Command::new("git")
                .current_dir(&path)
                .arg("reset")
                .arg("--hard")
                .arg("origin/master")
                .status()?;
        } else {
            Command::new("git")
                .current_dir(out_dir())
                .arg("clone")
                .arg(REPO_URL)
                .status()?;
        }
        Ok(())
    }

    pub fn try_build() -> PathBuf {
        cmake::Config::new(out_dir().join("openal-soft"))
            .define("ALSOFT_EXAMPLES", "OFF")
            .define("ALSOFT_CPUEXT_SSE", feature_cmake_str!("sse"))
            .define("ALSOFT_CPUEXT_SSE2", feature_cmake_str!("sse2"))
            .define("ALSOFT_CPUEXT_SSE3", feature_cmake_str!("sse3"))
            .define("ALSOFT_CPUEXT_SSE4_1", feature_cmake_str!("sse4_1"))
            .define("ALSOFT_CPUEXT_NEON", feature_cmake_str!("arm_neon"))
            .define("ALSOFT_BACKEND_PIPEWIRE", feature_cmake_str!("pipewire"))
            .define(
                "ALSOFT_BACKEND_PULSEAUDIO",
                feature_cmake_str!("pulseaudio")
            )
            .define("ALSOFT_BACKEND_ALSA", feature_cmake_str!("alsa"))
            .define("ALSOFT_BACKEND_OSS", feature_cmake_str!("oss"))
            .define("ALSOFT_BACKEND_SOLARIS", feature_cmake_str!("solaris"))
            .define("ALSOFT_BACKEND_SNDIO", feature_cmake_str!("sndio"))
            .define("ALSOFT_BACKEND_WINMM", feature_cmake_str!("winmm"))
            .define("ALSOFT_BACKEND_DSOUND", feature_cmake_str!("directsound"))
            .define("ALSOFT_BACKEND_JACK", feature_cmake_str!("jack"))
            .define("ALSOFT_BACKEND_COREAUDIO", feature_cmake_str!("coreaudio"))
            .define("ALSOFT_BACKEND_OBOE", feature_cmake_str!("oboe"))
            .define("ALSOFT_BACKEND_OPENSL", feature_cmake_str!("opensl"))
            .define("ALSOFT_BACKEND_PORTAUDIO", feature_cmake_str!("portaudio"))
            .define("ALSOFT_BACKEND_SDL3", feature_cmake_str!("sdl3"))
            .define("ALSOFT_BACKEND_SDL2", feature_cmake_str!("sdl2"))
            .define("ALSOFT_BACKEND_WAVE", feature_cmake_str!("wave"))
            .build()
    }
}

fn main() {
    #[cfg(feature = "build")]
    {
        cmake_build::try_clone_repo().expect("Failed to clone repository");

        let dst = cmake_build::try_build();
        println!("cargo:rustc-link-search=native={}", dst.display());
        if cfg!(target_os = "windows") {
            println!("cargo:rustc-link-lib=OpenAL32");
        } else {
            println!("cargo:rustc-link-lib=openal");
        }
    }
}
