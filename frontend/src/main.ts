import "virtual:windi.css"
import 'virtual:windi-devtools'
import 'vite/modulepreload-polyfill'

import App from './App.svelte'

const app = new App({
  target: document.getElementById('app') as HTMLElement
})

export default app
