import { invoke } from '@tauri-apps/api/tauri'
export * from './dark'

export function inv(name: string) {
  invoke('hello', { yourName: `123${name}` })
}
