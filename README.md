# binutils
CLI apps for controlling a Linux laptop backlight, volume, microphone, touchpad etc.

# Build dependencies
* `rust`
* `cargo`
* `just` command runner https://github.com/casey/just

# Installation
* on Arch Linux: `just prod-build-install`

# 3rd party dependencies
* `pactl` - PulseAudio
* `notify-send` - showing notifications
* `swaymsg` - interacting with Sway WM
* `light` - for controlling laptop brightness
