# 📝 SpeedyNote

_A lightweight, fast, and stylus-optimized note-taking app built for classic tablet PCs, low-resolution screens, and
vintage hardware._

如果您恰好不懂英文，请移步[中文README](./readme/zh_Hans.md)

<a href="https://hellogithub.com/repository/alpha-liu-01/SpeedyNote" target="_blank"><img src="https://abroad.hellogithub.com/v1/widgets/recommend.svg?rid=e86680d007424ab59d68d5e787ad5c12&claim_uid=e5oCIWstjbEUv9D" alt="Featured｜HelloGitHub" style="width: 250px; height: 54px;" width="250" height="54" /></a>

![cover](https://i.imgur.com/U161QSH.png)

---

## ✨ Features

- 🖊️ **Pressure-sensitive inking** with stylus support
- 📄 **Multi-page notebooks** with tabbed or scrollable page view
- 📌 **PDF background integration** with annotation overlay
- 🌀 **Dial UI + Joy-Con support** for intuitive one-handed control
- 🎨 **Per-page background styles**: grid, lined, or blank (customizable)
- 💾 **Portable `.spn` notebooks** for note storage
- 🔎 **Zoom, pan, thickness, and color preset switching** via dial
- 🗔 **Markdown sticky notes are supported** for text-based notes
- 💡 **Designed for low-spec devices** (133Hz Sample Rate @ Intel Atom N450)
- 🌏 **Supports multiple languages across the globe** (Covers half the global population)

---

## 📸 Screenshots

| Drawing                                  | Dial UI / Joycon Controls               | Overlay Grid Options                     |
| ---------------------------------------- | --------------------------------------- | ---------------------------------------- |
| ![draw](https://i.imgur.com/iARL6Vo.gif) | ![pdf](https://i.imgur.com/NnrqOQQ.gif) | ![grid](https://i.imgur.com/YaEdx1p.gif) |

---

## 🚀 Getting Started

### ✅ Requirements

- **Windows**: 7/8/10/11 (x86_64, ARM64)
- **Linux**: Ubuntu/Debian/Fedora/RedHat/ArchLinux/AlpineLinux (x86_64, ARM64, ARMv7)
- **Runtime**: Qt 6 (bundled in Windows releases)
- **Input**: Stylus input (Wacom recommended), Joy-Con controller support

### 📦 Installation

#### Linux Packages
Pre-built packages are available for multiple architectures:

**Debian/Ubuntu (.deb):**
```bash
sudo dpkg -i speedynote_*_amd64.deb    # For x86_64
sudo dpkg -i speedynote_*_arm64.deb    # For ARM64
sudo dpkg -i speedynote_*_armhf.deb    # For ARMv7
```

**Fedora/RHEL (.rpm):**
```bash
sudo rpm -i speedynote-*-x86_64.rpm    # For x86_64
sudo rpm -i speedynote-*-aarch64.rpm   # For ARM64
sudo rpm -i speedynote-*-armv7hl.rpm   # For ARMv7
```

**Arch Linux (.pkg.tar.zst):**
```bash
sudo pacman -U speedynote-*-x86_64.pkg.tar.zst    # For x86_64
sudo pacman -U speedynote-*-aarch64.pkg.tar.zst   # For ARM64
```

### 🛠️ Usage

1. **Launch** `SpeedyNote` from your applications menu or desktop shortcut
2. **Create/Open**: Click `Open PDF` to annotate a PDF or `New` for a blank notebook
3. **Draw**: Start writing/drawing using your stylus
4. **Control**: Use the **MagicDial** or **Joy-Con** for tools, zoom, and page navigation
5. **Save**: Close tabs to save as portable `.spn` packages

#### Alternative Workflow

1. **Right-click** any PDF file in your file manager
2. **Select** "Open with SpeedyNote"
3. **Automatically** create and open a `.spn` notebook package
4. **Reload**: Double-click the `.spn` file to resume editing with PDF background

---

## 🎮 Controller Support

SpeedyNote supports controller input, ideal for tablet users:

- ✅ **Left Joy-Con supported**
- 🎛️ Analog stick → Dial control
- 🔘 Buttons can be mapped to:
  - Control the dial with multiple features
  - Toggle fullscreen
  - Change color / thickness
  - Open control panel
  - Create/delete pages

> Long press + turn = hold-and-turn mappings

---

## 📁 Building From Source

#### Windows

[Windows Build Documentation](./docs/SpeedyNote_Windows_Build_en.md)  [Windows Build ARM64 Documentation](./docs/SpeedyNote_Windows_ARM64_Build_en.md)

#### Linux

1. run  `./build-package.sh`
2. Install the packages for your Linux distro.
   `.deb`, `rpm`, `.pkg.tar.zst` and `.apk` are tested and working.
