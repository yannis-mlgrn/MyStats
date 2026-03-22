<script setup>
const menuItems = [
  { name: 'Tableau de bord', icon: '📊', active: true },
  { name: 'Activités', icon: '📋' },
  { name: 'Objectifs', icon: '🎯' },
  { name: 'Équipement', icon: '👟' },
  { name: 'Paramètres', icon: '⚙️' }
]
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <div class="logo-icon">
        <svg viewBox="0 0 24 24" fill="var(--color-brand)" width="28" height="28">
          <path d="M12 2L2 22h20L12 2zm0 4.5l6.5 13.5h-13L12 6.5z"/>
        </svg>
      </div>
      <span class="logo-text">My Stats</span>
    </div>
    
    <nav class="sidebar-nav">
      <div 
        v-for="item in menuItems" 
        :key="item.name"
        class="nav-item"
        :class="{ active: item.active }"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-name">{{ item.name }}</span>
      </div>
    </nav>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width, 250px);
  background: var(--bg-sidebar, #fff);
  border-right: 1px solid var(--border-light, #e5e7eb);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  height: 100%;
}

.sidebar-header {
  height: 70px;
  display: flex;
  align-items: center;
  padding: 0 1.5rem;
  border-bottom: 1px solid var(--border-light, #e5e7eb);
  gap: 0.75rem;
}

.logo-text {
  font-weight: 700;
  font-size: 1.25rem;
  color: var(--text-main, #111827);
}

.sidebar-nav {
  flex: 1;
  padding: 1.5rem 0;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 0.85rem 1.5rem;
  cursor: pointer;
  color: var(--text-muted, #6b7280);
  font-size: 0.95rem;
  font-weight: 500;
  transition: all 0.2s ease;
}

.nav-item:hover {
  background: var(--bg-app, #f4f5f7);
  color: var(--text-main, #111827);
}

.nav-item.active {
  background: #eff6ff;
  color: var(--color-brand, #2563eb);
  font-weight: 600;
  border-right: 3px solid var(--color-brand, #2563eb);
}

.nav-icon {
  width: 24px;
  margin-right: 0.75rem;
  text-align: center;
  font-size: 1.1rem;
}
</style>
