<script setup lang="ts">
import { globalShortcut } from '@tauri-apps/api'
import type {
  RequestOption,
} from '@arco-design/web-vue/es/upload/interfaces'
import { Bucket, Region, copy, cos, imgList, isSupported, sendCopy } from '~/composables'
if ((window as any).__TAURI__) {
  globalShortcut.register('CmdOrControl+J', () => {
    sendCopy()
  })
}

const customRequest = (options: RequestOption) => {
  const {
    fileItem,
    onSuccess,
    onError,
    onProgress,
  } = options

  if (fileItem.name && fileItem.file) {
    cos.putObject({
      Bucket,
      Region,
      Key: fileItem.name, /* 存储在桶里的对象键（例如1.jpg，a/b/test.txt），必须字段 */
      StorageClass: 'STANDARD',
      Body: fileItem.file, // 上传文件对象
      onProgress(event) {
        let percent = 0
        if (event.total > 0)
          percent = (event.loaded / event.total) * 100

        onProgress(percent)
      },
    }).then((res) => {
      if (fileItem.name) {
        imgList.value.push({
          name: fileItem.name,
          url: `https://${res.Location}`,
          date: useDateFormat(useNow(), 'YYYY-MM-DD HH:mm:ss').value,
        })
      }
      if (isSupported)
        copy(`https://${res.Location}`)

      onSuccess()
      Message.success('上传成功')
    }).catch((err) => {
      onError()
      Message.error(`上传失败:${err.message}`)
    })
  }
  else {
    Message.error('文件类型错误')
  }
  return {
    abort() {
      if (fileItem.name) {
        cos.deleteObject({
          Bucket,
          Region,
          Key: fileItem.name,
        })
      }
    },
  }
}

// 判断是否为图片格式--img标签可打开的
function isImage(str: string) {
  const reg = /.(png||pdf|jpg|gif|jpeg|webp)$/
  return reg.test(str)
}
const fileList = $ref([])
const beforeUpload = (file: File) => {
  if (isImage(file.name)) { return Promise.resolve(true) }

  else {
    Message.error('文件类型错误,仅支持: png|pdf|svg|jpg|gif|jpeg|webp')
    return Promise.reject(new Error('文件类型错误,仅支持: png|pdf|svg|jpg|gif|jpeg|webp'))
  }
}
</script>

<template>
  <a-upload
    v-model:file-list="fileList" draggable list-type="picture" accept="image/*,.pdf"
    :custom-request="customRequest" @before-upload="beforeUpload"
  />
</template>
