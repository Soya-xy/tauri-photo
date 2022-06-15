<script setup lang="ts">
// 监听全局复制事件
import { appWindow } from '@tauri-apps/api/window'
import { listenClip } from './composables'
if ((window as any).__TAURI__)
  appWindow.listen('ClipboardEvent', listenClip)
</script>

<template>
  <main font-sans px4 text="center gray-700 dark:gray-200">
    <Menu />
    <router-view />
    <Footer />
  </main>
</template>
