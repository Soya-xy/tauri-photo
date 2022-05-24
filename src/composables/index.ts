import { invoke } from '@tauri-apps/api/tauri'
export * from './dark'
export const src = ref('')

export async function inv() {
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
