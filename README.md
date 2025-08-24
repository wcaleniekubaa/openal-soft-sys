# openal-soft-sys

Low-level Rust FFI bindings to the [OpenAL Soft](https://github.com/kcat/openal-soft) library.

## Installation

Add this line to your `Cargo.toml`:
```toml
[dependencies]
openal-soft-sys = "0.1.0"
```

## Usage

And then:

```rust
use openal_soft_sys::alc;
use std::ptr;

fn main() {
    // Open device
    let device = unsafe { alc::alcOpenDevice(ptr::null()) };
    assert!(!device.is_null(), "Failed to open device");

    // Create context
    let context = unsafe { alc::alcCreateContext(device, ptr::null()) };
    assert!(!context.is_null(), "Failed to create context");

    // ...

    // Destroy context and close device
    unsafe {
        alc::alcDestroyContext(context);
        alc::alcCloseDevice(device);
    }
}

```

## Feature flags

### Build options
- ✅ - `build` - Builds OpenAL Soft from source

### CPU features
- ✅ - `sse` - Enables SSE support
- ✅ - `sse2` - Enables SSE2 support
- ✅ - `sse3` - Enables SSE3 support
- ✅ - `sse4_1` - Enables SSE4.1 support
- ✅ - `sse_all` - Enables all SSE feature flags
- ✅ - `arm_neon` - Enables Neon support
- ✅ - `all_cpu_features` - Enables all CPU feature flags

### Audio backends
- ✅ - `pipewire` - Enables [Pipewire](https://www.pipewire.org) backend
- ✅ - `pulseaudio` - Enables [PulseAudio](https://wiki.freedesktop.org/www/Software/PulseAudio) backend
- ✅ - `alsa` - Enables [ALSA](https://www.alsa-project.org) backend
- ✅ - `oss` - Enables OSS backend
- ✅ - `solaris` - Enables Solaris backend
- ✅ - `sndio` - Enables [Sndio](https://sndio.org) backend
- ✅ - `winmm` - Enables Windows Multimedia backend
- ✅ - `directsound` - Enables [DirectSound](https://learn.microsoft.com/en-us/previous-versions/windows/desktop) backend
- ✅ - `jack` - Enables [JACK](https://jackaudio.org) backend
- ✅ - `coreaudio` - Enables [Core Audio](https://developer-mdn.apple.com/library/archive/documentation/MusicAudio/Conceptual/CoreAudioOverview/WhatisCoreAudio/WhatisCoreAudio.html) backend
- ✅ - `oboe` - Enables [Oboe](https://developer.android.google.cn/games/sdk/oboe?hl=en) backend
- ✅ - `opensl` - Enables [OpenSL](https://www.khronos.org/opensles) backend
- ✅ - `portaudio` - Enables [PortAudio](https://portaudio.com/docs/v19-doxydocs) backend
- ✅ - `sdl3` - Enables [SDL3](https://libsdl.org/) audio backend
- ✅ - `sdl2` - Enables [SDL2](https://libsdl.org/) audio backend
- ✅ - `wave` - Enables Wave Writer backend
- ✅ - `all_backends` - Enables all backends