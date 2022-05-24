<script setup lang="ts">
import { globalShortcut } from '@tauri-apps/api'
import { sendCopy } from '~/composables'

globalShortcut.register('CmdOrControl+J', () => {
  sendCopy()
})
</script>

<template>
  <Menu />
  <a-upload draggable action="/" />
</template>
