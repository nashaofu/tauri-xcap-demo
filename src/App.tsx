import { createSignal } from 'solid-js'
import logo from './assets/logo.svg'
import { invoke } from '@tauri-apps/api/tauri'
import './App.css'

function App() {
  const [result, setResult] = createSignal('')
  const [name, setName] = createSignal('')

  async function xcapTest() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setResult(await invoke('xcap_test'))
  }

  async function screenshotsTest() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setResult(await invoke('screenshots_test'))
  }

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <button onClick={screenshotsTest}>screenshots test</button>
      <button onClick={xcapTest}>xcap test</button>
      <p>{result()}</p>
    </div>
  )
}

export default App
