import type { TableData } from '@arco-design/web-vue'
import type { Event } from '@tauri-apps/api/helpers/event'
import { invoke } from '@tauri-apps/api/tauri'
import dayjs from 'dayjs'
import { v4 } from 'uuid'

export interface Image {
  width: number
  height: number
  bytes: [number]
}

export interface List extends TableData {
  key: string
  type: 'Image' | 'Text'
  content: Image | string
  time: string
}

export const history = ref<List[]>([])

export async function bytesToBlob(Image: Image): Promise<{ blob: Blob; src: string }> {
  const canvas = document.createElement('canvas')
  const context = canvas.getContext('2d')!
  canvas.width = Image.width
  canvas.height = Image.height
  const imgData = context.createImageData(Image.width, Image.height)
  imgData.data.set(Image.bytes)

  context.putImageData(imgData, 10, 10)

  return new Promise((resolve) => {
    canvas.toBlob(file => resolve({
      blob: file!,
      src: URL.createObjectURL(file!),
    }))
  })
}

export async function sendCopy() {
  const { Image } = await invoke('get_image')
  bytesToBlob(Image)
  history.value.unshift({
    key: v4(),
    content: Image,
    type: 'Image',
    time: dayjs().format('YYYY-MM-DD HH:mm'),
  })
}

export async function set_image(image: Image) {
  await invoke('set_image', { image })
}

export function listenClip(e: Event<{
  Image: Image
  Text: string
}>) {
  const type = Object.hasOwn(e.payload, 'Image') ? 'Image' : 'Text'
  history.value.unshift({
    key: v4(),
    content: e.payload[type],
    type,
    time: dayjs().format('YYYY-MM-DD HH:mm'),
  })
}
