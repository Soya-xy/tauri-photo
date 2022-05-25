<script setup lang='ts'>
import type { TableColumnData, TableData } from '@arco-design/web-vue'
import type { Image, List } from '~/composables'

import { history, set_image } from '~/composables'
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

const del = (e: TableData, idx: number) => {
  history.value.splice(idx, 1)
}

const copy = async (e: List) => {
  try {
    if (e.type === 'Text')
      await cp(e.content as string)
    else
      await set_image(e.content as Image)
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
      :columns="columns" :data="history" stripe :loading="loading" show-header :row-selection="{
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
        <Image v-if="record.type === 'Image'" :content="record.content" />
      </template>
      <template #action="{ record, rowIndex }">
        <a-space>
          <a-button type="primary" @click="copy(record)">
            Copy
          </a-button>
          <a-button status="danger" @click="del(record, rowIndex)">
            Delete
          </a-button>
        </a-space>
      </template>
    </a-table>
  </div>
</template>
