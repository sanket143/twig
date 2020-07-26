import { createApp } from "vue"
import App from "./app.vue"
import router from "./router"
import store from "./store"

import "@/assets/styles/index.css"

createApp(App).use(router).use(store).mount("#app")
