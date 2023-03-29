//import "uno.css"
//import 'virtual:unocss-devtools'
import "@unocss/reset/tailwind.css"
import 'vite/modulepreload-polyfill'

import App from './App.svelte'

const app = new App({
  target: document.getElementById('app') as HTMLElement
})

export default app
