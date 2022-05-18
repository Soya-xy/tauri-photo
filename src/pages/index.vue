<script setup lang="ts">
import { globalShortcut } from '@tauri-apps/api'
import { inv, name } from '~/composables'

globalShortcut.register('CmdOrControl+Alt+J', () => {
  inv('Soya')
})
</script>

<template>
  <div v-if="name">
    Not Found
    <Counter :initial="0" />
  </div>
  <div v-else>
    True
    <Counter :initial="0" />
  </div>
</template>
