<script setup>
import { ref, onMounted } from 'vue'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js'
import { Line } from 'vue-chartjs'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

const chartData = ref(null)
const chartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  interaction: {
    mode: 'index',
    intersect: false
  },
  scales: {
    y: {
      beginAtZero: true,
      grid: { color: '#f9fafb' },
      ticks: {
        color: '#9ca3af',
        font: { size: 10 }
      }
    },
    x: {
      grid: { display: false },
      ticks: {
        color: '#9ca3af',
        font: { size: 10 }
      }
    }
  },
  plugins: {
    legend: {
      position: 'top',
      align: 'end',
      labels: {
        boxWidth: 8,
        usePointStyle: true,
        font: { size: 10, weight: '600' }
      }
    },
    tooltip: {
      backgroundColor: '#1f2937',
      padding: 10,
      usePointStyle: true
    }
  },
  elements: {
    line: {
      tension: 0,
      borderWidth: 2
    },
    point: {
      radius: 0,
      hoverRadius: 5
    }
  }
})

const loading = ref(true)

const fetchHistory = async () => {
  try {
    const res = await fetch('/api/stats/history')
    if (!res.ok) throw new Error('Network response was not ok')
    const data = await res.json()

    const labels = data.map((w) => w.monday_date)
    const runningData = data.map((w) => w.running_km)
    const cyclingData = data.map((w) => w.cycling_km)
    const swimmingData = data.map((w) => w.swimming_m / 1000)

    chartData.value = {
      labels,
      datasets: [
        {
          label: 'Vélo',
          borderColor: '#3b82f6',
          backgroundColor: 'rgba(59, 130, 246, 0.05)',
          fill: true,
          data: cyclingData
        },
        {
          label: 'Course',
          borderColor: '#f59e0b',
          backgroundColor: 'rgba(245, 158, 11, 0.05)',
          fill: true,
          data: runningData
        },
        {
          label: 'Natation',
          borderColor: '#06b6d4',
          backgroundColor: 'rgba(6, 182, 212, 0.05)',
          fill: true,
          data: swimmingData
        }
      ]
    }
  } catch (error) {
    console.error('Error loading historical stats:', error)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchHistory()
})
</script>

<template>
  <div class="historical-chart-card">
    <div class="card-header">
      <h3 class="card-title">Historique d'activité</h3>
      <span class="card-subtitle">12 dernières semaines</span>
    </div>
    <div class="chart-container">
      <div v-if="loading" class="loading-state">Chargement de l'historique...</div>
      <Line v-else-if="chartData" :options="chartOptions" :data="chartData" />
    </div>
  </div>
</template>

<style scoped>
.historical-chart-card {
  background: var(--bg-card, #fff);
  border: 1px solid #f3f4f6;
  border-radius: 16px;
  padding: 1.5rem;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.05);
  width: 100%;
  height: 100%;
  transition: transform 0.2s ease;
  display: flex;
  flex-direction: column;
}

.historical-chart-card:hover {
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 1.5rem;
}

.card-title {
  font-size: 1.1rem;
  font-weight: 800;
  color: #111827;
  margin: 0;
}

.card-subtitle {
  font-size: 0.75rem;
  color: #6b7280;
  font-weight: 500;
}

.chart-container {
  flex-grow: 1;
  position: relative;
  min-height: 250px;
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
  font-size: 0.875rem;
}
</style>
