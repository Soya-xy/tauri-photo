<template>
  <section container mx-auto>
    <a-form-item field="name" label="上传快捷键">
      <a-input :model-value="keys.join('+')" @keydown="keyDown" @blur="clearKey" placeholder="请录制上传快捷键" />
    </a-form-item>
    <a-button mx-auto long @click="handleSubmit" type="primary" size="large">保存</a-button>
  </section>
</template>
<script setup lang="ts">
import { Message } from '@arco-design/web-vue';
let keys = $ref<string[]>([])
const current = $ref(new Set<string>())

const keyDown = (e: KeyboardEvent) => {
  console.log(e);
  if (aliasMap[e.code]) {
    current.add(aliasMap[e.code])
  } else {
    current.add(e.code.replace(/Key/, ''))
  }
  keys = Array.from(current)
}
const clearKey = () => {
  current.clear()
}

const aliasMap: Record<string, string> = {
  MetaRight: '⌘',
  AltRight: '⌥',
  ControlLeft: '⌃'
}


const handleSubmit = () => {
  Message.success('保存成功')
}

</script>
