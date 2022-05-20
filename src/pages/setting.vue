<script setup lang="ts">
import { Message } from '@arco-design/web-vue'
let keys = $ref<string[]>([])
const current = $ref(new Set<string>())

const aliasMap: Record<string, string> = {
  MetaRight: '⌘',
  AltRight: '⌥',
  ControlLeft: '⌃',
}
const keyDown = (e: KeyboardEvent) => {
  if (aliasMap[e.code])
    current.add(aliasMap[e.code])
  else
    current.add(e.code.replace(/Key/, ''))

  keys = Array.from(current)
}
const clearKey = () => {
  current.clear()
}

const handleSubmit = () => {
  Message.success('保存成功')
}
</script>

<template>
  <section container mx-auto>
    <a-form-item field="name" label="上传快捷键">
      <a-input :model-value="keys.join('+')" placeholder="请录制上传快捷键" @keydown="keyDown" @blur="clearKey" />
    </a-form-item>
    <a-button mx-auto long type="primary" size="large" @click="handleSubmit">
      保存
    </a-button>
  </section>
</template>
