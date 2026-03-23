<script setup>
import { ref, onMounted } from 'vue'
import Sidebar from './components/Sidebar.vue'
import WeeklyStatsCard from './components/WeeklyStatsCard.vue'
import HistoricalChart from './components/HistoricalChart.vue'
import SleepChart from './components/SleepChart.vue'
import ActivityGauges from './components/ActivityGauges.vue'

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
      <div class="dashboard-wrapper">
        <div v-if="loading" class="loading-state">
          <div class="loader"></div>
          Chargement de vos performances...
        </div>
        <div v-else-if="error" class="error-state">{{ error }}</div>

        <div v-else class="dashboard-content">
          <!-- TOP ROW: Activity Stats & History -->
          <div class="dashboard-row top-row">
            <div class="row-item stats-item">
              <WeeklyStatsCard :stats="stats" />
            </div>
            <div class="row-item history-item">
              <HistoricalChart />
            </div>
          </div>

          <!-- BOTTOM ROW: Sleep & Activity Rings -->
          <div class="dashboard-row bottom-row">
            <div class="row-item sleep-item">
              <SleepChart />
            </div>
            <div class="row-item rings-item">
              <ActivityGauges />
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap');

:root {
  --bg-app: #f8fafc;
  --bg-card: #ffffff;
  --bg-sidebar: #ffffff;
  --text-main: #1e293b;
  --text-muted: #64748b;
  --border-light: #f1f5f9;
  --color-brand: #3b82f6;
  --sidebar-width: 260px;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body,
html,
#app {
  font-family: 'Plus Jakarta Sans', sans-serif;
  color: var(--text-main);
  background-color: var(--bg-app);
  margin: 0;
  padding: 0 !important;
  width: 100%;
  height: 100%;
  display: block;
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
  overflow-y: auto;
  padding: 2rem;
}

.dashboard-wrapper {
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.dashboard-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  animation: slideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1);
}

.dashboard-row {
  display: flex;
  gap: 1.5rem;
}

.top-row .row-item {
  flex: 1;
  min-width: 0;
}

.bottom-row .sleep-item {
  flex: 7; /* 70% */
}

.bottom-row .rings-item {
  flex: 3; /* 30% */
}

@media (max-width: 1200px) {
  .bottom-row {
    flex-direction: column;
  }
}

@media (max-width: 1100px) {
  .top-row {
    flex-direction: column;
  }
}

/* LOADER & STATES */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 10rem 0;
  color: var(--text-muted);
  gap: 1rem;
}

.loader {
  width: 40px;
  height: 40px;
  border: 3px solid #e2e8f0;
  border-top-color: var(--color-brand);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }
@keyframes slideUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.error-state {
  padding: 2rem;
  background: #fef2f2;
  border: 1px solid #fee2e2;
  border-radius: 12px;
  color: #991b1b;
  text-align: center;
}
</style>
