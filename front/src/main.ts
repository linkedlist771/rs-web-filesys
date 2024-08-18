import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import Antd from 'ant-design-vue'
import App from './App.vue'
// import router from './router'  // 确保您已经设置了路由

// 导入 Ant Design Vue 的样式
import 'ant-design-vue/dist/antd.css'

const app = createApp(App)

app.use(createPinia())
// app.use(router)  // 使用路由
app.use(Antd)  // 使用 Ant Design Vue

app.mount('#app')
