# 🖥️ xresm – Resolution Manager for X11

> **CLI tool to quickly and painlessly set screen resolution for X11-based Linux systems.**

Tired of using `xrandr` and `cvt` every time your monitor misbehaves? `xresm` automates that whole dance — making screen resolution changes fast, easy, and reliable.

---

## 📦 Features

- ⚡ Automatically generates modelines via `cvt`
- 📡 Detects connected display outputs via `xrandr`
- 🧠 Applies new resolutions using `xrandr`'s `--newmode`, `--addmode`, and `--output`
- 🧪 Checks if dependencies (`xrandr`, `cvt`) are installed
- 🚀 Fast and minimal: written in safe Rust
- 🖥️ Currently supports single-monitor setups

---

## 📋 Requirements

- Linux system with **X11**
- `xrandr` and `cvt` installed and available in `$PATH`

### Debian / Ubuntu
```bash
sudo apt update
sudo apt install x11-xserver-utils
```

### Fedora / RHEL
```bash
sudo dnf install xorg-x11-server-utils
```

### Arch / Manjaro / Endeavor OS
```bash
sudo pacman -S xorg-xrandr
```

---

## 🧠 How It Works

`xresm` follows this sequence:

1. Checks for `xrandr` and `cvt`
2. Runs `cvt <width> <height>` to generate a modeline
3. Extracts the modeline from `cvt` output
4. Runs:
    - `xrandr --newmode "mode" ...`
    - `xrandr --addmode <output> "mode"`
    - `xrandr --output <output> --mode "mode"`
---

## 📈 Roadmap

- [X] Support for **multi-monitor setups** (apply same resolution to all; beta stage)
- [ ] Persistent mode (via `.xprofile`)
- [ ] Resolution presets / saving profiles

---
## 🔐 License:Custom Controlled Use License v1.1

You may:
- ✅ Use and share **unmodified** copies
- ✅ Submit PRs and commits for review
- ✅ Fork the project **privately** for learning/experimentation

You may NOT:
- ❌ Distribute modified versions
- ❌ Host or publish altered forks
- ❌ Merge changes without author approval

Author retains all rights and control.  
See [LICENSE](./LICENSE) file for details.

---

## 🙏 Credits

Created with love by [@MTalha-Codes](https://github.com/MTalha-Codes)  
If it saved you a headache — leave a ⭐ on GitHub!

