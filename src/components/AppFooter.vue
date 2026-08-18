<template>
  <footer class="app-footer">
    <p>
      <span>存储位置：</span>
      <strong>将在初始化本地数据库后显示</strong>
    </p>
    <p>
      <span>版本：</span>
      <strong>v0.1.0</strong>
      <button type="button" class="footer-update-btn">检查更新</button>
    </p>
  </footer>
</template>

<style scoped>
.footer-update-btn {
  padding-left: 10px;
  color: #2563eb;
  cursor: pointer;
  border-left: 1px solid #e2e8f0;
  background: transparent;
  font-size: 11.5px;
  font-weight: 500;
  transition: color 0.15s ease;
}

.footer-update-btn:hover {
  color: #1d4ed8;
  text-decoration: underline;
}
</style>
