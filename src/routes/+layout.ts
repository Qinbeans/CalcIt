import { PUBLIC_TAURI_DEV } from '$env/static/public'

export const prerender = true
export const ssr = PUBLIC_TAURI_DEV === 'true'