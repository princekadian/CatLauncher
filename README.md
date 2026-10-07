<p align="center">
  <img src="apps/app-frontend/src/assets/cat-logo.png" alt="Cat Launcher logo" width="128" height="128" />
</p>

<h1 align="center">Cat Launcher 🐱</h1>

<p align="center">
  A cute, cat-themed Minecraft launcher for Windows, macOS and Linux.<br />
  Fork of <a href="https://github.com/DIDIRUS4/AstralRinth">AstralRinth</a>, which is itself a fork of the <a href="https://github.com/modrinth/code">Modrinth App</a>.
</p>

---

## Contents
- [About](#about)
- [Where it comes from](#where-it-comes-from)
- [Features](#features)
- [Install](#install)
- [Build from source](#build-from-source)
- [What changed compared to AstralRinth](#what-changed-compared-to-astralrinth)
- [Data folder](#data-folder)
- [Credits](#credits)
- [License](#license)
- [Disclaimer](#disclaimer)

## About

Cat Launcher is a Minecraft launcher with a kitten theme: the launcher has a kitten icon and background, and every
click inside it plays a little meow (which you can turn down or off). Under the hood it is the full Modrinth App:
browse and install mods, modpacks, resource packs and shaders from [Modrinth](https://modrinth.com), manage
instances, and launch Minecraft with any mod loader.

## Where it comes from

Cat Launcher is a fork of a fork. All the hard work belongs to the projects below:

| Project | Author | What it is |
|---|---|---|
| [**Modrinth App**](https://github.com/modrinth/code) (codename *Theseus*) | [Modrinth](https://modrinth.com) / Rinth, Inc. and contributors | The original open-source launcher and monorepo this code comes from. |
| [**AstralRinth**](https://github.com/DIDIRUS4/AstralRinth) | [DIDIRUS4](https://github.com/DIDIRUS4) and contributors | A Modrinth App fork that adds offline accounts, removes ads and telemetry, and adds Discord Rich Presence. |
| [SmilerRyan/AstralRinth](https://github.com/SmilerRyan/AstralRinth) | [SmilerRyan](https://github.com/SmilerRyan) | The AstralRinth mirror this fork was cloned from (`beta` branch). |
| **Cat Launcher** (this repo) | [princekadian](https://github.com/princekadian) | The cat-themed changes listed below. |

The full commit history of the upstream projects is kept in this repository, so every original author stays credited.

## Features

### 🐱 Added in Cat Launcher
- **Cat Launcher branding:** new name everywhere (window title, installer, Start menu, settings, error messages).
- **Kitten logo:** a kitten app icon, splash screen and Settings → About logo.
- **Kitten background:** a soft, blurred kitten photo behind every page, under a theme-colored veil so all text stays readable in both dark and light themes.
- **Meow on click:** every left click inside the launcher plays a short meow.
  - Played natively from Rust (via [`rodio`](https://github.com/RustAudio/rodio)), so it works with every audio driver, including virtual devices like FxSound.
  - **Settings → Appearance** has a *Meow on click* toggle, a *Meow volume* slider (1–100%, in 1% steps) and a *Test meow* button.
- **Discord Rich Presence:** shows "Playing **Cat Launcher**" with the kitten logo.
- **Own data folder:** `%APPDATA%\CatLauncher`, with automatic migration from AstralRinth (see [Data folder](#data-folder)).
- **No auto-updates from AstralRinth:** update checks are disabled, so Cat Launcher never replaces itself with an AstralRinth release.

### Inherited from AstralRinth
- Offline (non-Microsoft) accounts alongside normal licensed Microsoft accounts.
- Statistics collection (telemetry) and personalized ads forced off.
- Ads removed from all launcher views.
- Discord Rich Presence with random status messages and an in-game timer.

### Inherited from the Modrinth App
- Browse and install mods, modpacks, resource packs, data packs and shaders from Modrinth.
- Instances with Fabric, Quilt, Forge and NeoForge, each with its own mods and settings.
- Automatic Java detection and installation.
- Import instances from other launchers (CurseForge, MultiMC, Prism, ATLauncher, GDLauncher).
- `.mrpack` modpack file support, logs viewer, crash reports and more.

## Install

Download the installer from the [Releases page](https://github.com/princekadian/CatLauncher/releases) if one is
available, or [build it yourself](#build-from-source).

- **Windows:** run `Cat Launcher_<version>_x64-setup.exe` (or the `.msi`).
  The installer isn't code-signed, so Windows SmartScreen may say "Windows protected your PC".
  Click **More info → Run anyway**.

## Build from source

### Requirements
- [Node.js](https://nodejs.org) 20 or newer and [pnpm](https://pnpm.io) 9 (`npm install -g pnpm@9`)
- [Rust](https://rustup.rs) (stable)
- **Windows:** [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with the
  **Desktop development with C++** workload, for example:
  ```
  winget install --id Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
  ```
- **Linux:** the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (WebKitGTK and friends).

### Commands
```bash
pnpm install

# Run the launcher in development mode
pnpm app:dev

# Build the installers
pnpm app:build
```

Installers are written to `target/release/bundle/` (`nsis/*.exe` and `msi/*.msi` on Windows).

> **Low on RAM?** Compiling uses a lot of memory. If the build fails with `memory allocation ... failed`,
> limit parallel jobs, for example `CARGO_BUILD_JOBS=4 pnpm app:build`.

## What changed compared to AstralRinth

| Area | Files |
|---|---|
| Name, window title, installer name, app ID | `apps/app/tauri.conf.json`, `tauri.macos.conf.json`, `tauri.linux.conf.json`, `apps/app-frontend/index.html`, UI text in `apps/app-frontend/src` |
| App icons | `apps/app/icons/*` (generated with `pnpm tauri icon`) |
| Splash screen and settings logo | `apps/app-frontend/src/components/ui/SplashScreen.vue`, `packages/assets/icons/astralrinth-logo.svg`, `apps/app-frontend/src/assets/cat-logo.png` |
| Kitten background | `apps/app-frontend/src/App.vue`, `apps/app-frontend/src/assets/cat-background.jpg` |
| Meow on click | `apps/app-frontend/src/helpers/meow.js`, `apps/app/src/api/utils.rs` (`play_meow`), `apps/app/build.rs`, `apps/app-frontend/src/assets/meow.mp3` |
| Meow settings | `apps/app-frontend/src/components/ui/settings/AppearanceSettings.vue` |
| Discord Rich Presence | `packages/app-lib/src/state/discord.rs` |
| Data folder and migration | `packages/app-lib/src/state/dirs.rs`, `packages/app-lib/src/state/db.rs`, `apps/app/src/main.rs` |
| Update checks disabled | `apps/app-frontend/src/helpers/update.js` |

## Data folder

Cat Launcher keeps instances, Minecraft files, accounts and settings in:

| OS | Folder |
|---|---|
| Windows | `%APPDATA%\CatLauncher` |
| macOS | `~/Library/Application Support/CatLauncher` |
| Linux | `~/.local/share/CatLauncher` |

If you used AstralRinth before, Cat Launcher moves your old `AstralRinthApp` folder to `CatLauncher` on first start
and updates the paths stored in its database, so your instances keep working. If the folder can't be moved
(for example because a file is in use), Cat Launcher keeps using the old folder so nothing is lost.

## Credits
- **[Modrinth](https://github.com/modrinth/code)** and all its contributors, for the Modrinth App.
- **[DIDIRUS4](https://github.com/DIDIRUS4/AstralRinth)** and the AstralRinth contributors.
- **[SmilerRyan](https://github.com/SmilerRyan/AstralRinth)** for the AstralRinth mirror.
- **[RustAudio/rodio](https://github.com/RustAudio/rodio)** for audio playback.

## License

Like the projects it is based on, Cat Launcher is free software licensed under the
**[GNU General Public License v3.0](apps/app/LICENSE)**. Each package in this monorepo has its own `LICENSE` file;
see [COPYING.md](COPYING.md).

Modrinth's name, logo and branding are trademarks of Rinth, Inc. Cat Launcher is not affiliated with or endorsed by
Modrinth, Mojang or Microsoft.

## Disclaimer

Like AstralRinth, Cat Launcher is intended for experimentation and educational purposes. It does not endorse
piracy: please buy Minecraft and use a licensed account to support its developers.
