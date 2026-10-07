// Cat Launcher: plays a short "meow" (assets/meow.mp3) on every click.
// Playback happens natively in the Rust backend so it works with every audio driver.
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const MIN_GAP_MS = 90
const STORAGE_KEY = 'cat-launcher-meow'

function loadSettings() {
  try {
    const saved = JSON.parse(localStorage.getItem(STORAGE_KEY))
    if (saved && typeof saved.enabled === 'boolean' && typeof saved.volume === 'number') {
      return saved
    }
  } catch {
    // ignore unreadable settings and use the defaults
  }
  return { enabled: true, volume: 60 }
}

/** Meow settings shown in Settings → Appearance: { enabled: boolean, volume: 0–100 } */
export const meowSettings = ref(loadSettings())

watch(
  meowSettings,
  (value) => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(value))
    } catch {
      // settings just won't be remembered
    }
  },
  { deep: true },
)

let lastPlayed = -Infinity

export function playMeow({ force = false } = {}) {
  if (!force && !meowSettings.value.enabled) return

  const now = performance.now()
  if (now - lastPlayed < MIN_GAP_MS) return
  lastPlayed = now

  invoke('plugin:utils|play_meow', { volume: meowSettings.value.volume / 100 }).catch((err) =>
    console.warn('Could not play meow sound', err),
  )
}
