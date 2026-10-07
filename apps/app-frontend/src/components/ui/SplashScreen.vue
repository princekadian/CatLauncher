<template>
  <div v-if="!hidden" class="splash-screen dark" :class="{ 'fade-out': doneLoading }">
    <div v-if="os !== 'MacOS'" class="app-buttons">
      <button class="btn icon-only transparent" icon-only @click="() => getCurrent().minimize()">
        <MinimizeIcon />
      </button>
      <button class="btn icon-only transparent" @click="() => getCurrent().toggleMaximize()">
        <MaximizeIcon />
      </button>
      <button class="btn icon-only transparent" @click="handleClose">
        <XIcon />
      </button>
    </div>
    <div class="app-logo-wrapper" data-tauri-drag-region>
      <div class="app-logo">
        <img :src="catLogo" alt="Cat Launcher" class="app-logo-image" draggable="false" />
        <span class="app-logo-text">Cat Launcher</span>
      </div>
      <ProgressBar class="loading-bar" :progress="Math.min(loadingProgress, 100)" />
      <span v-if="message">{{ message }}</span>
    </div>
    <div class="gradient-bg" data-tauri-drag-region></div>
    <div class="cube-bg"></div>
    <div class="base-bg"></div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import ProgressBar from '@/components/ui/ProgressBar.vue'
import { loading_listener } from '@/helpers/events.js'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { XIcon, MaximizeIcon, MinimizeIcon } from '@modrinth/assets'
import { getOS } from '@/helpers/utils.js'
import { useLoading } from '@/store/loading.js'
import catLogo from '@/assets/cat-logo.png'

const doneLoading = ref(false)
const loadingProgress = ref(0)
const hidden = ref(false)
const message = ref()

const loading = useLoading()

watch(loading, (newValue) => {
  if (!newValue.barEnabled) {
    if (loading.loading) {
      loadingProgress.value = 0
      fakeLoadingIncrease()
    } else {
      loadingProgress.value = 100
      doneLoading.value = true

      setTimeout(() => {
        hidden.value = true
        loading.setEnabled(true)
      }, 250)
    }
  }
})

function fakeLoadingIncrease() {
  if (loadingProgress.value < 95) {
    setTimeout(() => {
      loadingProgress.value += 1
      fakeLoadingIncrease()
    }, 5)
  }
}

const os = ref('')
getOS().then((x) => (os.value = x))

loading_listener(async (e) => {
  if (e.event.type === 'directory_move') {
    loadingProgress.value = 100 * (e.fraction ?? 1)
    message.value = 'Updating app directory...'
  } else if (e.event.type === 'launcher_update') {
    loadingProgress.value = 100 * (e.fraction ?? 1)
    message.value = 'Updating Cat Launcher...'
  } else if (e.event.type === 'checking_for_updates') {
    loadingProgress.value = 100 * (e.fraction ?? 1)
    message.value = 'Checking for updates...'
  }
})

const handleClose = async () => {
  await getCurrentWindow().close()
}
</script>

<style scoped lang="scss">
.splash-screen {
  transition: opacity 0.25s ease-in-out;
  opacity: 1;

  &.fade-out {
    opacity: 0;
  }
}

.app-buttons {
  position: absolute;
  right: 0;
  z-index: 9999;
  display: flex;
}

.app-logo-wrapper {
  position: absolute;
  height: 100vh;
  width: 100%;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  gap: 1rem;

  z-index: 9998;
}

.app-logo {
  display: flex;
  align-items: center;
  gap: 1rem;
  pointer-events: none;
}

.app-logo-image {
  height: 4rem;
  width: 4rem;
  border-radius: 1rem;
}

.app-logo-text {
  font-size: 2.25rem;
  font-weight: 800;
  color: var(--color-contrast);
}

.loading-bar {
  max-width: 20rem;
}

.gradient-bg {
  position: absolute;
  height: 100vh;
  width: 100vw;
  background: linear-gradient(180deg, rgba(66, 131, 92, 0.275) 0%, rgba(17, 35, 43, 0.5) 97.29%),
    linear-gradient(0deg, rgba(22, 24, 28, 0.64), rgba(22, 24, 28, 0.64));
  z-index: 9997;
}

.cube-bg {
  position: absolute;

  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);

  width: 180vw;
  height: 180vh;
  opacity: 0.8;
  background: #16181c url('@/assets/loading/cube.png') center no-repeat;
  background-size: contain;

  z-index: 9996;
}

.base-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: var(--color-bg);
  z-index: 9995;
}
</style>
