<script setup lang="ts">
import { globalShortcut } from '@tauri-apps/api'
import { inv } from '~/composables'

globalShortcut.register('CmdOrControl+Alt+J', () => {
  inv('Soya')
})
</script>

<template>
  <a-upload draggable action="/" />

</template>
