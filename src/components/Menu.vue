<script setup lang='ts'>
import { isDark } from '~/composables'
const router = useRouter()
const route = useRoute()
let keys = $ref('')
const clickMenu = (e: string) => {
  router.push(e)
}
watchEffect(
  () => keys = route.path,
)
</script>

<template>
  <div mb4>
    <a-menu :theme="isDark ? 'dark' : 'light'" mode="horizontal" :selected-keys="[keys]" @menu-item-click="clickMenu">
      <a-menu-item key="/">
        Upload
      </a-menu-item>
      <a-menu-item key="/list">
        Clipboard
      </a-menu-item>
      <a-menu-item key="/setting">
        Setting
      </a-menu-item>
    </a-menu>
  </div>
</template>

<style scoped>
.menu-demo {
  box-sizing: border-box;
  width: 100%;
  padding: 40px;
  background-color: var(--color-neutral-2);
}
</style>
