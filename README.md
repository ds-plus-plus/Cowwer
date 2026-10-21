# Cowwer 
**Play the game in your browser:** https://pixelds.itch.io/cowwer

Cowwer is a grid-based 2D game submitted for a 3 day Mini Jam. The entire project was designed and built from scratch in 48 hours. It was built against a strict jam deadline, the priority was just getting a working game out the door. This initial commit is the raw prototype exactly as it was submitted.

## Tech Stack
- Rust (2024 Edition)
- Bevy 0.16.1
- Bevy Rapier2D

## Running Locally 
```
git clone https://github.com/ds-plus-plus/Cowwer.git
cd Cowwer 
cargo run
```

On Linux systems, ensure you have bevy-specific dependencies. Please see the official documentation for any errors
https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

### Debian Based
```
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
# optional if you disabled the wayland feature
sudo apt-get install libwayland-dev libxkbcommon-dev
```

### Fedora
```
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
# optional if you disabled the wayland feature
sudo dnf install wayland-devel libxkbcommon-devel
```

### Arch Based
`sudo pacman -S libx11 pkgconf alsa-lib libxcursor libxrandr libxi`

## Assets & Credits
- Programming & Design: ds-plus-plus
- Art & Audio: All assets were created by me during the 48-hour window. Feel free to use them as desired!
