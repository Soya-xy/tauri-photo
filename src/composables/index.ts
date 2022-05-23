import { invoke } from '@tauri-apps/api/tauri'
export * from './dark'

export function inv(name: string) {
  invoke('hello', { yourName: `123${name}` }).then((res) => {
    console.log(res, 'res')

    console.log(JSON.parse(res as string), '123')
  })
}
