<script setup>
import { ref, computed, onMounted } from 'vue'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  LineElement,
  PointElement,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js'
import { Bar } from 'vue-chartjs'
import ChartDataLabels from 'chartjs-plugin-datalabels'

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  LineElement,
  PointElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

const rawData = ref([])
const displayMode = ref('total')

const formatSimpleDuration = (hours) => {
  const h = Math.floor(hours)
  const m = Math.round((hours - h) * 60)
  return `${h}h${m.toString().padStart(2, '0')}`
}

const formatDurationLong = (hours) => {
  const h = Math.floor(hours)
  const m = Math.round((hours - h) * 60)
  if (h === 0) return `${m}min`
  return `${h}h ${m}min`
}

// Stats computations
const averageDuration = computed(() => {
  if (rawData.value.length === 0) return 0
  const valid = rawData.value.filter(d => d.total_sleep_seconds > 0)
  if (valid.length === 0) return 0
  const sum = valid.reduce((acc, d) => acc + d.total_sleep_seconds, 0)
  return sum / valid.length / 3600
})

const averageScore = computed(() => {
  if (rawData.value.length === 0) return 0
  const valid = rawData.value.filter(d => d.quality_score > 0)
  if (valid.length === 0) return 0
  const sum = valid.reduce((acc, d) => acc + d.quality_score, 0)
  return Math.round(sum / valid.length)
})

const chartOptions = computed(() => ({
  responsive: true,
  maintainAspectRatio: false,
  interaction: {
    mode: 'index',
    intersect: false,
  },
  scales: {
    y: {
      stacked: displayMode.value === 'phases',
      beginAtZero: true,
      title: { display: false },
      grid: { color: '#cbd5e1' },
      ticks: { color: '#64748b', font: { size: 10 } }
    },
    y1: {
      type: 'linear',
      display: displayMode.value === 'phases',
      position: 'right',
      min: 0,
      max: 100,
      title: { display: false },
      grid: { drawOnChartArea: false },
      ticks: { color: '#64748b', font: { size: 10 } }
    },
    x: {
      stacked: displayMode.value === 'phases',
      grid: { display: false },
      ticks: { color: '#64748b', font: { size: 10 } }
    }
  },
  plugins: {
    datalabels: {
      display: (context) => {
        return displayMode.value === 'total' && context.dataset.label === 'Durée Totale'
      },
      anchor: 'end',
      align: 'top',
      offset: 2,
      color: '#1e293b',
      font: { size: 10, weight: '700' },
      formatter: (value) => formatSimpleDuration(value)
    },
    legend: {
      position: 'top',
      align: 'end',
      display: displayMode.value === 'phases',
      labels: {
        boxWidth: 8,
        usePointStyle: true,
        font: { size: 10 }
      }
    },
    tooltip: {
      backgroundColor: '#1f2937',
      padding: 10,
      usePointStyle: true,
      callbacks: {
        label: (context) => {
          if (context.dataset.yAxisID === 'y1') {
            return `${context.dataset.label}: ${context.raw}`
          }
          return `${context.dataset.label}: ${formatDurationLong(context.raw)}`
        }
      }
    }
  },
  layout: {
    padding: { top: 25 }
  }
}))

const chartData = computed(() => {
  if (rawData.value.length === 0) return null

  const sorted = [...rawData.value].reverse()
  const labels = sorted.map(d => {
    const date = new Date(d.date)
    return date.toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit' })
  })

  const datasets = []

  if (displayMode.value === 'phases') {
    datasets.push(
      {
        label: 'Profond',
        backgroundColor: '#1e3a8a',
        data: sorted.map(d => d.deep_sleep_seconds / 3600),
        borderRadius: 4,
        order: 2,
        datalabels: { display: false }
      },
      {
        label: 'Léger',
        backgroundColor: '#3b82f6',
        data: sorted.map(d => d.light_sleep_seconds / 3600),
        order: 2,
        datalabels: { display: false }
      },
      {
        label: 'REM',
        backgroundColor: '#8b5cf6',
        data: sorted.map(d => d.rem_sleep_seconds / 3600),
        order: 2,
        datalabels: { display: false }
      },
      {
        label: 'Éveillé',
        backgroundColor: '#f87171',
        data: sorted.map(d => d.awake_seconds / 3600),
        order: 2,
        datalabels: { display: false }
      }
    )

    datasets.push({
      label: 'Score Qualité',
      type: 'line',
      borderColor: '#10b981',
      borderWidth: 2,
      tension: 0,
      pointRadius: 3,
      pointBackgroundColor: '#10b981',
      fill: false,
      data: sorted.map(d => d.quality_score),
      yAxisID: 'y1',
      order: 1,
      datalabels: { display: false }
    })
  } else {
    const colors = sorted.map(d => {
      if (d.quality_score >= 80) return '#10b981'
      if (d.quality_score >= 60) return '#f59e0b'
      return '#ef4444'
    })

    datasets.push({
      label: 'Durée Totale',
      backgroundColor: colors,
      data: sorted.map(d => d.total_sleep_seconds / 3600),
      borderRadius: 6,
      datalabels: { display: true }
    })
  }

  return { labels, datasets }
})

const fetchSleepData = async () => {
  try {
    const res = await fetch('http://localhost:3000/api/stats/sleep')
    const data = await res.json()
    rawData.value = data
  } catch (e) {
    console.error('Error fetching sleep data:', e)
  }
}

onMounted(fetchSleepData)
</script>

<template>
  <div class="sleep-chart-card">
    <div class="card-header">
      <div class="header-left">
        <h3 class="card-title">Sommeil & Récupération</h3>
        <span class="card-subtitle">Analyse hebdomadaire des cycles</span>
      </div>

      <div class="toggle-group">
        <button
          :class="['toggle-btn', { active: displayMode === 'total' }]"
          @click="displayMode = 'total'"
        >
          Total
        </button>
        <button
          :class="['toggle-btn', { active: displayMode === 'phases' }]"
          @click="displayMode = 'phases'"
        >
          Phases
        </button>
      </div>
    </div>

    <div class="chart-container">
      <Bar v-if="chartData" :data="chartData" :options="chartOptions" :plugins="[ChartDataLabels]" />
      <div v-else class="loading-placeholder">Chargement du sommeil...</div>
    </div>

    <div class="banner-layout">
      <div class="averages-banner">
        <div class="avg-item">
          <span class="avg-label">Moyenne Sommeil</span>
          <span class="avg-value">{{ formatDurationLong(averageDuration) }}</span>
        </div>
        <div class="avg-divider"></div>
        <div class="avg-item">
          <span class="avg-label">Score Moyen</span>
          <span
class="avg-value" :class="{
            'text-green': averageScore >= 80,
            'text-orange': averageScore >= 60 && averageScore < 80,
            'text-red': averageScore < 60
          }">{{ averageScore }}/100</span>
        </div>
      </div>

      <div v-if="displayMode === 'total'" class="chart-footer">
        <div class="legend-item"><span class="dot green"></span> Qualité ++</div>
        <div class="legend-item"><span class="dot orange"></span> Moyenne</div>
        <div class="legend-item"><span class="dot red"></span> Fatigué</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sleep-chart-card {
  background: var(--bg-card, #fff);
  border-radius: 16px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.05);
  padding: 1.5rem;
  border: 1px solid #f3f4f6;
  transition: transform 0.2s ease;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.sleep-chart-card:hover {
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
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

.toggle-group {
  display: flex;
  background: #f3f4f6;
  padding: 3px;
  border-radius: 8px;
}

.toggle-btn {
  padding: 4px 12px;
  font-size: 0.75rem;
  font-weight: 600;
  border: none;
  background: transparent;
  color: #6b7280;
  cursor: pointer;
  border-radius: 10px;
  transition: all 0.2s ease;
}

.toggle-btn.active {
  background: #fff;
  color: #111827;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.chart-container {
  height: 250px;
  position: relative;
  flex-grow: 1;
}

.banner-layout {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 1.25rem;
  gap: 2rem;
}

.averages-banner {
  display: flex;
  align-items: center;
  background: #f8fafc;
  border-radius: 12px;
  padding: 0.75rem 1.25rem;
  gap: 1.5rem;
  box-shadow: inset 0 2px 4px rgba(0,0,0,0.02);
}

.avg-item {
  display: flex;
  flex-direction: column;
}

.avg-label {
  font-size: 0.6rem;
  font-weight: 700;
  text-transform: uppercase;
  color: #94a3b8;
  letter-spacing: 0.05em;
}

.avg-value {
  font-size: 0.95rem;
  font-weight: 800;
  color: #1e293b;
}

.avg-divider {
  width: 1px;
  height: 20px;
  background: #e2e8f0;
}

.text-green { color: #10b981; }
.text-orange { color: #f59e0b; }
.text-red { color: #ef4444; }

.chart-footer {
  display: flex;
  gap: 1.25rem;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.65rem;
  font-weight: 600;
  color: #94a3b8;
}

.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.dot.green { background: #10b981; }
.dot.orange { background: #f59e0b; }
.dot.red { background: #ef4444; }

.loading-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #9ca3af;
  font-size: 0.875rem;
}
</style>
