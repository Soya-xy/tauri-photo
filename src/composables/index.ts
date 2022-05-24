import type { Event } from '@tauri-apps/api/helpers/event'
import { invoke } from '@tauri-apps/api/tauri'
export * from './dark'

export const src = ref('')
export async function sendCopy() {
  const { Image } = await invoke('get_image')
  const canvas = document.createElement('canvas')
  const context = canvas.getContext('2d')!
  const imgData = context.createImageData(Image.width, Image.height)
  for (let i = 0; i < Image.bytes.length; i++)
    imgData.data[i] = Image.bytes[i]

  context.putImageData(imgData, 10, 10)
  canvas.toBlob((blob) => {
    src.value = URL.createObjectURL(blob!)
  })
}

export function listenClip<T>(e: Event<T>) {
  // console.log(e, '123')
  return e
}
