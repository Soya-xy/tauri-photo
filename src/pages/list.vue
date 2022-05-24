<script setup lang='ts'>
import type { TableColumnData } from '@arco-design/web-vue'
const { copy: cp } = useClipboard()

const columns: TableColumnData[] = [
  {
    title: 'Type',
    slotName: 'type',
  },
  {
    title: 'Content',
    slotName: 'content',
  },
  {
    title: 'Time',
    dataIndex: 'time',
    width: 180,
    align: 'center',

  },
  {
    title: 'Action',
    align: 'center',
    width: 120,
    slotName: 'action',
  },
]

const data = [{
  key: '1',
  type: 'Text',
  content: 'Hello Wor',
  time: '2020-01-01 14:32:32',
}, {
  key: '2',
  type: 'Image',
  content: 'https://avatars2.githubusercontent.com/u/24394918?s=460&v=4',
  time: '2020-01-01 14:32:32',
}]
const loading = $ref(false)

let selectItem = $ref<string[]>([])
let selectAll = $ref<boolean>(false)
const select = (e: string[] | boolean) => {
  if (typeof e === 'boolean') { selectAll = e }
  else {
    selectAll = e.length > 1
    selectItem = e
  }
}

const del = (e) => {
  console.log(e)
}

const copy = async (e) => {
  try {
    if (e.type === 'Text')
      await cp(e.content)
    else
      await set_image(e.bytes)
    Message.success('Copy success')
  }
  catch (error) {
    Message.error('Copy failed')
  }
}
</script>

<template>
  <div mb2 w-full text-right h40px>
    <a-button v-if="selectAll" status="danger">
      All Delete
    </a-button>
  </div>
  <div>
    <a-table
      :columns="columns" :data="data" stripe :loading="loading" show-header :row-selection="{
        type: 'checkbox',
        showCheckedAll: true,
      }" @select="select" @select-all="select"
    >
      <template #type="{ record }">
        <a-tag color="#168cff">
          {{ record.type }}
        </a-tag>
      </template>
      <template #content="{ record }">
        <span v-if="record.type === 'Text'" break-all>{{ record.content }}</span>
        <a-image v-if="record.type === 'Image'" :src="record.content" width="150" />
      </template>
      <template #action="{ record }">
        <a-space>
          <a-button type="primary" @click="copy(record)">
            Copy
          </a-button>
          <a-button status="danger" @click="del(record)">
            Delete
          </a-button>
        </a-space>
      </template>
    </a-table>
  </div>
</template>
