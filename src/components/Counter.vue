<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
const props = defineProps<{
  initial: number
}>()

invoke('my_custom_command')
const { count, inc, dec } = useCounter(props.initial)
</script>

<template>
  <div>
    {{ count }}
    <button class="inc" @click="inc() && invoke('my_custom_command', { invokeMessage: count })">
      +
    </button>
    <button class="dec" @click="dec() && invoke('my_custom_command', { invokeMessage: count })">
      -
    </button>
  </div>
</template>
