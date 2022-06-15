<script setup lang='ts'>
import { Waterfall } from 'vue-waterfall-plugin-next'
import { Bucket, Region, copy, cos, imgList, isDark } from '~/composables'
import 'vue-waterfall-plugin-next/style.css'
function onDownLoad(url: string, name: string) {
  const a = document.createElement('a')
  // 将链接地址字符内容转变成blob地址
  fetch(url).then(res => res.blob()).then((blob) => {
    a.href = URL.createObjectURL(blob)
    a.download = name || '' // 下载文件的名字
    document.body.appendChild(a)
    a.click()

    // 在资源下载完成后 清除 占用的缓存资源
    window.URL.revokeObjectURL(a.href)
    document.body.removeChild(a)
  })
}

function copyUrl(url: string) {
  copy(url).then(() => {
    Message.success('复制成功')
  }).catch(() => {
    Message.error('复制失败')
  })
}

function deleteItem(Key: string, k: number, all = false) {
  cos.deleteObject({
    Bucket,
    Region,
    Key,
  }, (err) => {
    if (err) { Message.error(`删除失败:${err.message}`) }
    else {
      if (!all) {
        imgList.value.splice(k, 1)
        Message.success('删除成功')
      }
    }
  })
}

const isFile = (e: string) => {
  if (e.endsWith('.svg'))
    return 'i-vscode-icons:file-type-svg'
  else if (e.endsWith('.pdf'))
    return 'i-vscode-icons:file-type-pdf2'
  return false
}

function sure() {
  Modal.confirm({
    title: 'Delete All Files?',
    content: 'Are you sure you want to delete all files?',
    onOk() {
      imgList.value.map((v) => {
        cos.deleteObject({
          Bucket,
          Region,
          Key: v.name,
        })
        return []
      })
      imgList.value = []
    },
  })
}
</script>

<template>
  <div v-if="imgList.length > 0">
    <a-button status="danger" absolute right-4 top-5 @click="sure">
      清空
    </a-button>
    <Waterfall
      :background-color="isDark ? '#121212' : 'white'" :list="imgList" img-selector="url" :delay="500"
      :breakpoints="{
        800: { //当屏幕宽度小于等于800
          rowPerView: 2,
        },
        500: { //当屏幕宽度小于等于500
          rowPerView: 1,
        },
      }"
    >
      <template #item="{ item, url, index }">
        <a-image
          v-if="isFile(item.name) === false" :src="url" :title="item.name" :description="item.date"
          footer-position="outer"
        >
          <template #extra>
            <div class="actions actions-outer">
              <a-tooltip content="Download">
                <span icon-btn i-carbon:download @click="onDownLoad(url, item.name)" />
              </a-tooltip>
              <a-tooltip content="Copy Url">
                <span ml2 icon-btn i-carbon:copy-link @click="copyUrl(item.url)" />
              </a-tooltip>
              <a-tooltip content="Delete This">
                <a-popconfirm content="Are you sure you want to delete?" @ok="deleteItem(item.name, index)">
                  <span ml2 icon-btn i-carbon:trash-can />
                </a-popconfirm>
              </a-tooltip>
            </div>
          </template>
        </a-image>
        <template v-else>
          <i icon-btn w-20 h-20 :class="isFile(item.name)" />
          <div class="arco-image-footer" prefix-cls="arco-image">
            <div class="arco-image-footer-caption">
              <div class="arco-image-footer-caption-title">
                {{ item.name }}
              </div>
              <div class="arco-image-footer-caption-description">
                {{ item.date }}
              </div>
            </div>
            <div class="arco-image-footer-extra">
              <div class="actions actions-outer">
                <a-tooltip content="Download">
                  <span icon-btn i-carbon:download @click="onDownLoad(url, item.name)" />
                </a-tooltip>
                <a-tooltip content="Copy Url">
                  <span ml2 icon-btn i-carbon:copy-link @click="copyUrl(item.url)" />
                </a-tooltip>
                <a-tooltip content="Delete This">
                  <a-popconfirm content="Are you sure you want to delete?" @ok="deleteItem(item.name, index)">
                    <span ml2 icon-btn i-carbon:trash-can />
                  </a-popconfirm>
                </a-tooltip>
              </div>
            </div>
          </div>
        </template>
      </template>
    </Waterfall>
  </div>
  <a-empty v-else />
</template>

<style scoped>
:deep(.arco-image-img) {
  margin: 0 auto;
}

:deep(.arco-image-footer-caption) {
  text-align: start;
}
</style>
