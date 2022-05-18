<script setup lang="ts">
import { inv } from '~/composables'

let initial = $ref('12')

setInterval(() => {
  initial += '32'
}, 3000)
</script>

<template>
  <div>
    {{ initial }}
    <button class="inc" @click="inv(initial)">
      +
    </button>
    <button class="inc" @click="inv(initial)">
      -
    </button>
  </div>
</template>
