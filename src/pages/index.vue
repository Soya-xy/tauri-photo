<script setup lang="ts">
import { name } from '~/composables'
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
