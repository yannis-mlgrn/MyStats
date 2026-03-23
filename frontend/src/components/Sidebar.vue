<script setup>
defineProps({
  isOpen: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['close'])

const menuItems = [
  { name: 'Tableau de bord', icon: '📊', active: true },
  { name: 'Activités', icon: '📋' },
  { name: 'Objectifs', icon: '🎯' },
  { name: 'Équipement', icon: '👟' },
  { name: 'Paramètres', icon: '⚙️' }
]
</script>

<template>
  <aside class="sidebar" :class="{ 'is-mobile-open': isOpen }">
    <div class="sidebar-header">
      <div class="logo-icon">
        <svg viewBox="0 0 24 24" fill="var(--color-brand)" width="28" height="28">
          <path d="M12 2L2 22h20L12 2zm0 4.5l6.5 13.5h-13L12 6.5z" />
        </svg>
      </div>
      <span class="logo-text">My Stats</span>
      <button class="close-mobile-btn" @click="emit('close')">✕</button>
    </div>

    <nav class="sidebar-nav">
      <div
        v-for="item in menuItems"
        :key="item.name"
        class="nav-item"
        :class="{ active: item.active }"
        @click="emit('close')"
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
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 100;
}

.sidebar-header {
  height: 70px;
  display: flex;
  align-items: center;
  padding: 0 1.5rem;
  border-bottom: 1px solid var(--border-light, #e5e7eb);
  gap: 0.75rem;
  position: relative;
}

.close-mobile-btn {
  display: none;
  position: absolute;
  right: 1rem;
  top: 50%;
  transform: translateY(-50%);
  background: transparent;
  border: none;
  font-size: 1.25rem;
  color: var(--text-muted);
  cursor: pointer;
  padding: 0.5rem;
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

@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    transform: translateX(-100%);
    box-shadow: 10px 0 30px rgba(0, 0, 0, 0.1);
  }

  .sidebar.is-mobile-open {
    transform: translateX(0);
  }

  .close-mobile-btn {
    display: block;
  }
}
</style>
