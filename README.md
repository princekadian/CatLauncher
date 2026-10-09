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

You can build the installer in two ways:

- **[Option A: on GitHub](#option-a-build-the-installer-on-github-no-setup)**. No setup. GitHub's servers build
  installers for Windows, macOS and Linux, and you download them.
- **[Option B: on your own PC](#option-b-build-the-installer-on-your-pc)**. Needs about 10 GB of tools and
  disk space, but gives you the installer locally and lets you test changes quickly.

### Option A: build the installer on GitHub (no setup)

The workflow in [`.github/workflows/theseus-release.yml`](.github/workflows/theseus-release.yml) builds the
installers automatically.

1. **Fork** this repository on GitHub (or use your own copy of it).
2. Open your fork's **Actions** tab. If GitHub asks, click **I understand my workflows, go ahead and enable them**.
3. Start a build in one of these ways:
   - **By hand:** in the Actions tab, pick **Cat Launcher build** on the left, click **Run workflow**, choose a
     branch and click the green **Run workflow** button.
   - **By pushing a branch** whose name starts with `feature` (for example `feature-my-change`). This only
     starts a build if the push changes files in `apps/app`, `apps/app-frontend`, `packages/app-lib`,
     `packages/daedalus`, `packages/assets`, `packages/ui`, `packages/utils` or the workflow file.
   - **By pushing a tag** that starts with `v` or `build`, for example:
     ```bash
     git tag v0.9.205
     git push origin v0.9.205
     ```
4. Wait for the run to finish. A build with no cache takes about 20–40 minutes; later builds are faster.
5. Open the finished run and scroll to **Artifacts**. Download:
   - **`windows-latest`**: the `.exe` installer (`nsis` folder) and the `.msi` installer (`msi` folder)
   - **`macos-latest`**: the `.dmg`
   - **`ubuntu-latest`**: the `.AppImage`, `.deb` and `.rpm`

   Artifacts are `.zip` files. Extract them to get the installers. GitHub deletes them after 90 days, so attach the
   ones you want to keep to a [release](https://github.com/princekadian/CatLauncher/releases/new).

### Option B: build the installer on your PC

These steps are for **Windows 10/11**. macOS and Linux notes follow after them.

#### 1. Install the tools (one time only)

Open **PowerShell** and run each command. Accept any administrator (UAC) prompts.

1. **Git**, to download the code:
   ```powershell
   winget install --id Git.Git -e
   ```
2. **Node.js 20 or newer**, to build the launcher's interface:
   ```powershell
   winget install --id OpenJS.NodeJS.LTS -e
   ```
3. **Visual Studio Build Tools with the C++ workload** (about 6 GB). Rust needs Microsoft's C++ linker and the
   Windows SDK:
   ```powershell
   winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
   ```
   If you already have Visual Studio (Community, Professional...), you can instead open the
   **Visual Studio Installer**, click **Modify**, and tick **Desktop development with C++**. Installing Visual
   Studio without that workload is **not** enough.
4. **Rust** (the stable toolchain, about 1.5 GB):
   ```powershell
   winget install --id Rustlang.Rustup -e
   ```
5. **Close PowerShell and open a new window** so it picks up the new tools. Then turn on
   [pnpm](https://pnpm.io), which comes with Node.js:
   ```powershell
   corepack enable
   ```
   If `corepack` fails with a permission error, run `npm install -g pnpm@9` instead.
6. Check that everything is there. Each command should print a version number:
   ```powershell
   git --version
   node --version
   pnpm --version
   rustc --version
   cargo --version
   ```

The installer also needs **Microsoft Edge WebView2**. Windows 10 (recent updates) and Windows 11 already include it.

#### 2. Download the code

```powershell
git clone https://github.com/princekadian/CatLauncher.git
cd CatLauncher
```

To update an existing copy later, run `git pull` inside the `CatLauncher` folder.

#### 3. Install the JavaScript dependencies

```powershell
pnpm install
```

The first run takes a few minutes. Run it again whenever you pull new changes.

#### 4. Build the installer

```powershell
pnpm --filter=@modrinth/app run tauri build --config "tauri-release.conf.json"
```

This is the same command that GitHub Actions uses. It:
1. builds the interface (`apps/app-frontend`),
2. compiles the launcher in release mode (`apps/app`, `packages/app-lib`, `packages/daedalus`), and
3. packages the result into installers. The first time, it automatically downloads the NSIS and WiX tools that
   the `.exe` and `.msi` installers need.

The **first build takes 15–40 minutes** depending on your PC and uses about 8–10 GB in the `target` folder. Later
builds only recompile what changed and are much faster.

#### 5. Find the installer

When the build prints `Finished ... bundles at:`, your installers are in:

| File | Location |
|---|---|
| `.exe` installer (recommended) | `target\release\bundle\nsis\Cat Launcher_<version>_x64-setup.exe` |
| `.msi` installer | `target\release\bundle\msi\Cat Launcher_<version>_x64_en-US.msi` |
| The launcher itself, without installing | `target\release\Cat Launcher.exe` |

The installer installs for all users, so it asks for administrator permission. It isn't code-signed, so Windows
SmartScreen may warn about it. See [Install](#install).

#### Run the launcher without building an installer

To try changes quickly, start the launcher in development mode. It reloads the interface when you save a file:

```powershell
pnpm app:dev
```

Development builds use the same data folder (`%APPDATA%\CatLauncher`) as the installed launcher.

#### Changing the version number

The version shown in the installer's file name and in the launcher comes from `"version"` in
[`apps/app/tauri.conf.json`](apps/app/tauri.conf.json). Change it before building a new release.

#### macOS and Linux

- **macOS:** install the Xcode command line tools (`xcode-select --install`), Node.js, pnpm and
  [Rust](https://rustup.rs), then follow steps 2–4. The `.dmg` is written to `target/release/bundle/dmg/`.
  To build a universal (Intel + Apple Silicon) app the way GitHub does, run
  `rustup target add aarch64-apple-darwin x86_64-apple-darwin` and add `--target universal-apple-darwin` to the
  build command. The output then goes to `target/universal-apple-darwin/release/bundle/`.
- **Linux (Debian/Ubuntu):** install the system libraries, then Node.js, pnpm and [Rust](https://rustup.rs), and
  follow steps 2–4:
  ```bash
  sudo apt-get update
  sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev pkg-config libayatana-appindicator3-dev librsvg2-dev libasound2-dev
  ```
  The `.AppImage`, `.deb` and `.rpm` go to `target/release/bundle/appimage/`, `deb/` and `rpm/`. For other
  distributions, see the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

#### Troubleshooting

| Problem | Fix |
|---|---|
| `linker 'link.exe' not found` or `error: linking with link.exe failed` | The Visual Studio **C++ workload** is missing. Repeat step 1.3, then open a new PowerShell window. |
| `'pnpm' / 'cargo' is not recognized` | Close and reopen PowerShell after installing. If it still fails, restart Windows so the `PATH` updates. |
| `memory allocation of ... bytes failed`, or the PC freezes while compiling | Compiling uses a lot of RAM. Limit parallel jobs: run `$env:CARGO_BUILD_JOBS = 4` in PowerShell, then the build command again. Use `2` on PCs with 8 GB RAM. |
| Errors mentioning `DATABASE_URL` or `sqlx` | The launcher builds from the saved queries in `packages/app-lib/.sqlx`. Make sure you have no `DATABASE_URL` environment variable set (`Remove-Item Env:DATABASE_URL` in PowerShell). |
| `failed to bundle project` while downloading NSIS/WiX | A firewall or antivirus blocked the download. Allow it, or retry on another network. |
| Build fails right after `git pull` | Run `pnpm install` again. If Rust errors persist, run `cargo clean` and rebuild. |
| The installer says another copy is running | Close Cat Launcher (check Task Manager for `Cat Launcher.exe`), then run the installer again. |

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
