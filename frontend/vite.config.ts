import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({ plugins:[vue(),tailwindcss()], server:{proxy:{'/api':{target:'http://127.0.0.1:3000',changeOrigin:true},'/api/v1/ws':{target:'ws://127.0.0.1:3000',ws:true}}} })
