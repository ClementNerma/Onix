import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'
import { config } from 'dotenv'

export default ({ mode }) => {
  config({ path: `./.env.${mode}` })

  return defineConfig({
    plugins: [svelte()],
    server: {
      port: 5170,
    },
  })
}
