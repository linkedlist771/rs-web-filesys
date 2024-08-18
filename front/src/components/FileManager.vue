
<template>
    <div class="file-manager">
      <a-row :gutter="[16, 16]">
        <a-col :span="24">
          <a-upload
            :customRequest="customUpload"
            :showUploadList="false"
            @change="handleUploadChange"
          >
            <a-button type="primary" :icon="h(UploadOutlined)">Upload File</a-button>
          </a-upload>
          <a-progress v-if="isUploading" :percent="uploadProgress" style="margin-top: 10px;" />
        </a-col>
        <a-col :span="24">
          <a-table :columns="columns" :data-source="files" :loading="loading">
            <template #bodyCell="{ column, record }">
              <template v-if="column.key === 'action'">
                <a-space>
                  <a-button type="link" @click="downloadFile(record)">Download</a-button>
                  <a-popconfirm
                    title="Are you sure you want to delete this file?"
                    @confirm="deleteFile(record)"
                  >
                    <a-button type="link" danger>Delete</a-button>
                  </a-popconfirm>
                </a-space>
              </template>
            </template>
          </a-table>
        </a-col>
      </a-row>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted, h } from 'vue';
  import { message, Progress as AProgress } from 'ant-design-vue';
  import { UploadOutlined } from '@ant-design/icons-vue';
  import { BASE_URL } from '@/config/constants';
  import axios from 'axios';
  
  
  const files = ref([]);
  const loading = ref(false);
  const uploadProgress = ref(0);
  const isUploading = ref(false);
  
  const columns = [
    {
      title: 'File Name',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'Action',
      key: 'action',
    },
  ];
  
  const fetchFiles = async () => {
    loading.value = true;
    try {
      const response = await axios.get(`${BASE_URL}/list`);
      files.value = response.data.map((file, index) => ({
        key: index,
        name: file,
      }));
    } catch (error) {
      message.error('Failed to fetch files');
    } finally {
      loading.value = false;
    }
  };
  
  const customUpload = async ({ file, onSuccess, onError }) => {
    const formData = new FormData();
    formData.append('file', file);
    try {
      isUploading.value = true;
      uploadProgress.value = 0;
      await axios.post(`${BASE_URL}/upload`, formData, {
        onUploadProgress: (progressEvent) => {
          const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
          uploadProgress.value = percentCompleted;
        }
      });
      onSuccess();
      message.success('File uploaded successfully');
      fetchFiles();
    } catch (error) {
      onError(error);
      message.error('Failed to upload file');
    } finally {
      isUploading.value = false;
    }
  };
  
  const handleUploadChange = (info) => {
    if (info.file.status === 'done') {
      message.success(`${info.file.name} file uploaded successfully`);
    } else if (info.file.status === 'error') {
      message.error(`${info.file.name} file upload failed.`);
    }
  };
  
  const downloadFile = async (record) => {
    try {
      const response = await axios.get(`${BASE_URL}/download/${record.name}`, {
        responseType: 'blob',
      });
      const url = window.URL.createObjectURL(new Blob([response.data]));
      const link = document.createElement('a');
      link.href = url;
      link.setAttribute('download', record.name);
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    } catch (error) {
      message.error('Failed to download file');
    }
  };
  
  const deleteFile = async (record) => {
    try {
      await axios.delete(`${BASE_URL}/delete/${record.name}`);
      message.success('File deleted successfully');
      fetchFiles();
    } catch (error) {
      message.error('Failed to delete file');
    }
  };
  
  onMounted(() => {
    fetchFiles();
  });
  </script>
  
  <style lang="scss" scoped>
  .file-manager {
    max-width: 800px;
    margin: 0 auto;
  }
  </style>
  
  