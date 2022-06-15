import COS from 'cos-js-sdk-v5'

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

export const Bucket = 'photoserver-1254285921' /* 存储桶，必须字段 */
export const Region = 'ap-beijing' /* 存储桶所在地域，必须字段 */
const SecretId = 'KIDf6IFSQOi02g5EBJKXUCiz1rSpxrWHFF5'
const SecretKey = 'GXZNxIOJ1UigPwEH7h27xXv4xd4GCsJ'
const c = '41'
const d = '6b'
// 初始化实例
export const cos = new COS({
  SecretId: hexToString(c) + SecretId,
  SecretKey: hexToString(d) + SecretKey,
})
