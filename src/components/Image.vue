<script setup lang='ts'>
import type { Image } from '~/composables'
import { bytesToBlob } from '~/composables'
const { content } = defineProps<{ content: Image }>()
let src = $ref('')
bytesToBlob(content).then((res) => {
  src = res.src
})
</script>

<template>
  <a-image :src="src" width="150" />
</template>
