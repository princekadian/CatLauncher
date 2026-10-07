use serde::{Deserialize, Serialize};
use theseus::{
    handler,
    prelude::{CommandPayload, DirectoryInfo},
};

use crate::api::Result;
use dashmap::DashMap;
use std::path::PathBuf;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("utils")
        .invoke_handler(tauri::generate_handler![
            get_artifact,
            get_os,
            should_disable_mouseover,
            highlight_in_folder,
            open_path,
            show_launcher_logs_folder,
            progress_bars_list,
            get_opening_command,
            play_meow
        ])
        .build()
}

#[tauri::command]
pub async fn get_artifact(downloadurl: &str, filename: &str, ostype: &str, autoupdatesupported: bool) -> Result<()> {
    theseus::download::init_download(downloadurl, filename, ostype, autoupdatesupported).await;
    Ok(())
}

/// Gets OS
#[tauri::command]
pub fn get_os() -> OS {
    #[cfg(target_os = "windows")]
    let os = OS::Windows;
    #[cfg(target_os = "linux")]
    let os = OS::Linux;
    #[cfg(target_os = "macos")]
    let os = OS::MacOS;
    os
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
}

// Lists active progress bars
// Create a new HashMap with the same keys
// Values provided should not be used directly, as they are not guaranteed to be up-to-date
#[tauri::command]
pub async fn progress_bars_list(
) -> Result<DashMap<uuid::Uuid, theseus::LoadingBar>> {
    let res = theseus::EventState::list_progress_bars().await?;
    Ok(res)
}

// cfg only on mac os
// disables mouseover and fixes a random crash error only fixed by recent versions of macos
#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn should_disable_mouseover() -> bool {
    // We try to match version to 12.2 or higher. If unrecognizable to pattern or lower, we default to the css with disabled mouseover for safety
    let os = os_info::get();
    if let os_info::Version::Semantic(major, minor, _) = os.version() {
        if *major >= 12 && *minor >= 3 {
            // Mac os version is 12.3 or higher, we allow mouseover
            return false;
        }
    }
    true
}
#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn should_disable_mouseover() -> bool {
    false
}

#[tauri::command]
pub fn highlight_in_folder(path: PathBuf) {
    let res = opener::reveal(path);

    if let Err(e) = res {
        tracing::error!("Failed to highlight file in folder: {}", e);
    }
}

#[tauri::command]
pub fn open_path(path: PathBuf) {
    let res = opener::open(path);

    if let Err(e) = res {
        tracing::error!("Failed to open path: {}", e);
    }
}

#[tauri::command]
pub fn show_launcher_logs_folder() {
    let path = DirectoryInfo::launcher_logs_dir().unwrap_or_default();
    // failure to get folder just opens filesystem
    // (ie: if in debug mode only and launcher_logs never created)
    open_path(path);
}

// Get opening command
// For example, if a user clicks on an .mrpack to open the app.
// This should be called once and only when the app is done booting up and ready to receive a command
// Returns a Command struct- see events.js
#[tauri::command]
#[cfg(target_os = "macos")]
pub async fn get_opening_command(
    state: tauri::State<'_, crate::macos::deep_link::InitialPayload>,
) -> Result<Option<CommandPayload>> {
    let payload = state.payload.lock().await;

    return if let Some(payload) = payload.as_ref() {
        tracing::info!("opening command {payload}");

        Ok(Some(handler::parse_command(payload).await?))
    } else {
        Ok(None)
    };
}

#[tauri::command]
#[cfg(not(target_os = "macos"))]
pub async fn get_opening_command() -> Result<Option<CommandPayload>> {
    // Tauri is not CLI, we use arguments as path to file to call
    let cmd_arg = std::env::args_os().nth(1);

    tracing::info!("opening command {cmd_arg:?}");

    let cmd_arg = cmd_arg.map(|path| path.to_string_lossy().to_string());
    if let Some(cmd) = cmd_arg {
        tracing::debug!("Opening command: {:?}", cmd);
        return Ok(Some(handler::parse_command(&cmd).await?));
    }
    Ok(None)
}

// helper function called when redirected by a weblink (ie: modrith://do-something) or when redirected by a .mrpack file (in which case its a filepath)
// We hijack the deep link library (which also contains functionality for instance-checking)
pub async fn handle_command(command: String) -> Result<()> {
    tracing::info!("handle command: {command}");
    Ok(theseus::handler::parse_and_emit_command(&command).await?)
}

/// Cat Launcher: the click "meow", embedded in the app.
const MEOW_SOUND: &[u8] = include_bytes!("../../../app-frontend/src/assets/meow.mp3");

/// Plays the meow sound through the system's default audio output at `volume` (0.0 to 1.0).
/// Audio is handled natively (not by the webview) so it works with every audio driver.
/// invoke('plugin:utils|play_meow', { volume })
#[tauri::command]
pub fn play_meow(volume: f32) {
    static MEOW_PLAYER: std::sync::OnceLock<std::sync::Mutex<std::sync::mpsc::Sender<f32>>> =
        std::sync::OnceLock::new();

    let volume = if volume.is_finite() { volume.clamp(0.0, 1.0) } else { 0.6 };
    if volume == 0.0 {
        return;
    }

    let sender = MEOW_PLAYER.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::channel::<f32>();
        std::thread::spawn(move || meow_player_thread(rx));
        std::sync::Mutex::new(tx)
    });
    if let Ok(sender) = sender.lock() {
        let _ = sender.send(volume);
    }
}

fn meow_player_thread(rx: std::sync::mpsc::Receiver<f32>) {
    use rodio::{Decoder, OutputStream, Source};

    // The output stream must live on this thread; reopen it if the audio device goes away
    let mut output: Option<(OutputStream, rodio::OutputStreamHandle)> = None;
    while let Ok(volume) = rx.recv() {
        for _ in 0..2 {
            if output.is_none() {
                output = OutputStream::try_default().ok();
            }
            let Some((_, handle)) = &output else { break };
            let Ok(source) = Decoder::new(std::io::Cursor::new(MEOW_SOUND)) else {
                break;
            };
            match handle.play_raw(source.convert_samples().amplify(volume)) {
                Ok(()) => break,
                Err(e) => {
                    tracing::warn!("Could not play meow sound: {e}");
                    output = None;
                }
            }
        }
    }
}
