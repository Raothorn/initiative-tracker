/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'
import { io } from 'socket.io-client'


// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

const app = createApp(App)

registerPlugins(app)

app.mount('#app')

// Socket
const socket = io("http://localhost/socket.io:9001/", {});

socket.on("connect", () => {
  console.log("socket connected")
});

