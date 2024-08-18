
<!-- App.vue -->
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
.layout {
  min-height: 100vh;
}

.header {
  display: flex;
  align-items: center;
  .logo {
    color: white;
    font-size: 1.5em;
    font-weight: bold;
  }
}

.content {
  padding: 24px;
  background: #fff;
}

.footer {
  text-align: center;
}
</style>
