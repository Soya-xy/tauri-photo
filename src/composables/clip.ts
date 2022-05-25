import type { Event } from '@tauri-apps/api/helpers/event'
import { invoke } from '@tauri-apps/api/tauri'

interface Image {
  width: number
  height: number
  bytes: [number]
}

interface List {
  key: number
  type: 'Image' | 'Text'
  content: Image | string
  time: Date
}

export const history = ref<List[]>([])

export async function sendCopy() {
  const { Image } = await invoke('get_image')
  const canvas = document.createElement('canvas')
  const context = canvas.getContext('2d')!
  const imgData = context.createImageData(Image.width, Image.height)

  for (let i = 0; i < Image.bytes.length; i++)
    imgData.data[i] = Image.bytes[i]

  context.putImageData(imgData, 10, 10)

  return canvas.toBlob(blob => URL.createObjectURL(blob!))
}

export async function set_image(image: number[]) {
  await invoke('set_image', { image })
}

export function listenClip(e: Event<{
  Image: Image
  Text: string
}>) {
  const type = Object.hasOwn(e.payload, 'Image') ? 'Image' : 'Text'
  history.value.unshift({
    key: new Date().getTime(),
    content: e.payload[type],
    type,
    time: new Date(),
  })
}
