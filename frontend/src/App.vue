<script setup>
import { ref, onMounted } from 'vue'
import Sidebar from './components/Sidebar.vue'
import TopNav from './components/TopNav.vue'
import WeeklyStatsCard from './components/WeeklyStatsCard.vue'

const stats = ref({
  running_km: 0,
  cycling_km: 0,
  swimming_m: 0
})
const loading = ref(true)
const error = ref(null)

const fetchStats = async () => {
  try {
    const res = await fetch('http://localhost:3000/api/stats/weekly')
    if (!res.ok) throw new Error('Erreur de chargement des stats')
    stats.value = await res.json()
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchStats()
})
</script>

<template>
  <div class="app-container">
    <Sidebar />

    <main class="main-content">
      <TopNav />

      <div class="dashboard-center">
        
        <div v-if="loading" class="loading-state">Chargement...</div>
        <div v-else-if="error" class="error-state">{{ error }}</div>
        
        <div v-else class="center-wrapper">
          <WeeklyStatsCard :stats="stats" />
        </div>
        
      </div>
    </main>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

:root {
  --bg-app: #f4f5f7;
  --bg-card: #ffffff;
  --bg-sidebar: #ffffff;
  --text-main: #111827;
  --text-muted: #6b7280;
  --border-light: #e5e7eb;
  --color-brand: #2563eb; /* Blue DA */
  --sidebar-width: 250px;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body, html, #app {
  font-family: 'Inter', sans-serif;
  color: var(--text-main);
  background-color: var(--bg-app);
  margin: 0;
  padding: 0 !important;
  width: 100%;
  height: 100%;
  max-width: none !important;
  display: block !important;
  overflow: hidden;
}

.app-container {
  display: flex;
  height: 100vh;
  width: 100%;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* CENTER DASHBOARD */
.dashboard-center {
  flex: 1;
  display: flex;
  align-items: flex-start; /* Centré vers le haut */
  justify-content: center;
  padding: 2rem 2rem 2rem 2rem; /* Reduced top margin to standard 2rem */
  overflow-y: auto;
}

.loading-state, .error-state {
  text-align: center;
  font-size: 1.1rem;
  color: var(--text-muted);
}

.center-wrapper {
  width: 100%;
  max-width: 800px;
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
