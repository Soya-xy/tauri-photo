export * from './dark'
export * from './clip'
export * from './list'

export function hexToString(str: string) {
  let val = ''
  const arr = str.split(',')
  for (let i = 0; i < arr.length; i++)
    val += String.fromCharCode(parseInt(arr[i], 16))
  return val
}
