<template>
  <a-config-provider :theme="{ token: { colorPrimary: '#1890ff' } }">
    <a-layout class="layout">
      <a-layout-header class="header">
        <div class="logo">File Manager</div>
      </a-layout-header>
      <a-layout-content class="content">
        <FileManager />
      </a-layout-content>
      <a-layout-footer class="footer">
        File Manager ©2024 Created by LLinkedlist771
      </a-layout-footer>
    </a-layout>
  </a-config-provider>
</template>


<script setup>
import { ref } from 'vue';
import FileManager from '@/components/FileManager.vue';
</script>

<style lang="scss">
html, body {
  margin: 0;
  padding: 0;
  height: 100%;
}

#app {
  height: 100%;
}

.layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  align-items: center;
  padding: 0 24px;
  background: #001529;
  position: fixed;
  width: 100%;
  z-index: 1000;
  
  .logo {
    color: white;
    font-size: 1.5em;
    font-weight: bold;
  }
}

.content {
  flex: 1;
  padding-top: 64px; // Header height
  background: #f0f2f5;
  display: flex;
  flex-direction: column;
  .content-wrapper {
    max-width: 100%;
    height: 100%;
    padding: 24px;
    box-sizing: border-box;
  }
}

.footer {
  text-align: center;
  background: #f0f2f5;
  padding: 24px 50px;
}

@media (max-width: 768px) {
  .header {
    padding: 0 16px;
  }

  .content {
    .content-wrapper {
      padding: 16px;
    }
  }

  .footer {
    padding: 16px;
  }
}
</style>

