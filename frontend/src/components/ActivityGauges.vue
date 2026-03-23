<script setup>
import { ref, computed, onMounted } from 'vue'

const rawData = ref([])
const loading = ref(true)
const isEditingGoal = ref(false)
const newGoal = ref(10000)

const latestDay = computed(() => rawData.value[0] || null)

// Helper for RHR color (lower is better -> greener)
const getRHRColor = (val) => {
  if (!val || val === 0) return '#94a3b8'
  if (val <= 55) return '#10b981' // Excellent (Green)
  if (val <= 65) return '#34d399' // Good
  if (val <= 75) return '#f59e0b' // Average (Orange)
  return '#ef4444' // High (Red)
}

const gauges = computed(() => {
  if (!latestDay.value) return []
  const d = latestDay.value

  return [
    {
      id: 'rhr',
      label: 'FC Repos',
      value: d.resting_heart_rate || 0,
      min: 40,
      max: 100,
      unit: 'BPM',
      color: getRHRColor(d.resting_heart_rate),
      description: 'Plus bas du jour'
    },
    {
      id: 'steps',
      label: 'Objectif Pas',
      value: d.steps || 0,
      min: 0,
      max: d.step_goal || 10000,
      unit: 'PAS',
      color: '#f59e0b', // Orange for steps
      description: `Cible : ${d.step_goal?.toLocaleString()}`
    }
  ]
})

const fetchData = async () => {
  try {
    const res = await fetch('http://localhost:3000/api/stats/sleep')
    const data = await res.json()
    rawData.value = data
    if (latestDay.value) {
      newGoal.value = latestDay.value.step_goal
    }
  } catch (e) {
    console.error('Error fetching activity gauges:', e)
  } finally {
    loading.value = false
  }
}

const updateGoal = async () => {
  if (!latestDay.value) return
  try {
    const res = await fetch('http://localhost:3000/api/stats/step-goal', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        date: latestDay.value.date,
        goal: parseInt(newGoal.value)
      })
    })
    if (res.ok) {
      isEditingGoal.value = false
      fetchData()
    }
  } catch (e) {
    console.error('Error updating step goal:', e)
  }
}

// SVG Gauge Path calculation (semi-circle)
const getGaugePath = (value, min, max) => {
  // Ensure we don't divide by zero and clamp percent [0, 1]
  const range = max - min || 1
  const percent = Math.min(Math.max((value - min) / range, 0), 1)

  const radius = 40
  const centerX = 50
  const centerY = 50
  const startAngle = -180
  const endAngle = startAngle + (percent * 180)

  const startRad = (startAngle * Math.PI) / 180
  const endRad = (endAngle * Math.PI) / 180

  const x1 = centerX + radius * Math.cos(startRad)
  const y1 = centerY + radius * Math.sin(startRad)
  const x2 = centerX + radius * Math.cos(endRad)
  const y2 = centerY + radius * Math.sin(endRad)

  return `M ${x1} ${y1} A ${radius} ${radius} 0 0 1 ${x2} ${y2}`
}

onMounted(fetchData)
</script>

<template>
  <div class="activity-gauges-card">
    <div class="card-header">
      <h3 class="card-title">Indicateurs Santé</h3>
      <span class="card-date">Aujourd'hui</span>
    </div>

    <div v-if="!loading && latestDay" class="gauges-container">
      <div v-for="gauge in gauges" :key="gauge.id" class="gauge-wrapper">
        <div class="gauge-item">
          <div class="gauge-visual">
            <svg viewBox="0 0 100 60" class="gauge-svg">
              <path
                d="M 10 50 A 40 40 0 0 1 90 50"
                class="gauge-bg"
              />
              <path
                :d="getGaugePath(gauge.value, gauge.min, gauge.max)"
                class="gauge-fill"
                :stroke="gauge.color"
              />
            </svg>
            <div class="gauge-center">
              <span class="center-value">{{ gauge.value ? gauge.value.toLocaleString() : '--' }}</span>
              <span class="center-unit">{{ gauge.unit }}</span>
            </div>
          </div>
          <div class="gauge-info">
            <div class="label-row">
              <span class="gauge-label">{{ gauge.label }}</span>
              <button v-if="gauge.id === 'steps'" class="edit-btn" @click="isEditingGoal = !isEditingGoal">
                <span v-if="!isEditingGoal">⚙️</span>
                <span v-else>✖</span>
              </button>
            </div>

            <!-- Specific display for steps goal editing -->
            <template v-if="gauge.id === 'steps' && isEditingGoal">
              <div class="edit-mode">
                <input
                  v-model="newGoal"
                  type="number"
                  class="goal-input"
                  @keyup.enter="updateGoal"
                />
                <button class="save-btn" @click="updateGoal">OK</button>
              </div>
            </template>
            <template v-else>
              <span class="gauge-desc">{{ gauge.description }}</span>
            </template>
          </div>
        </div>
        <div v-if="gauge.id === 'rhr'" class="divider"></div>
      </div>
    </div>

    <div v-else-if="loading" class="loading-state">
      Analyse des indicateurs...
    </div>
  </div>
</template>

<style scoped>
.activity-gauges-card {
  background: var(--bg-card, #fff);
  border-radius: 16px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.05);
  padding: 1.5rem;
  border: 1px solid #f3f4f6;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.card-header {
  margin-bottom: 0.5rem;
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.card-title {
  font-size: 1rem;
  font-weight: 800;
  color: #111827;
  margin: 0;
}

.card-date {
  font-size: 0.7rem;
  color: #94a3b8;
  font-weight: 600;
  text-transform: uppercase;
}

.gauges-container {
  display: flex;
  flex-direction: column;
  justify-content: center; /* Vertical alignment */
  flex-grow: 1;
  gap: 1rem;
}

.gauge-wrapper {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.gauge-item {
  display: flex;
  align-items: center;
  gap: 2rem;
  padding: 0.5rem 0;
}

.gauge-visual {
  width: 140px; /* Increased size */
  height: 85px;
  position: relative;
}

.gauge-svg {
  width: 100%;
  height: 100%;
}

.gauge-bg {
  fill: none;
  stroke: #f1f5f9;
  stroke-width: 10;
  stroke-linecap: round;
}

.gauge-fill {
  fill: none;
  stroke-width: 10;
  stroke-linecap: round;
  transition: all 0.8s cubic-bezier(0.4, 0, 0.2, 1);
}

.gauge-center {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  align-items: center;
}

.center-value {
  font-size: 1.25rem; /* Larger text */
  font-weight: 800;
  color: #1e293b;
  line-height: 1;
}

.center-unit {
  font-size: 0.65rem;
  color: #94a3b8;
  font-weight: 700;
  text-transform: uppercase;
  margin-top: 2px;
}

.gauge-info {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.label-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.gauge-label {
  font-size: 1.1rem; /* Larger labels */
  font-weight: 800;
  color: #334155;
}

.gauge-desc {
  font-size: 0.8rem;
  color: #64748b;
  font-weight: 500;
}

.divider {
  height: 1px;
  background: #f1f5f9;
  margin: 0.5rem 0;
  width: 80%;
  align-self: center;
}

.edit-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  opacity: 0.5;
  transition: opacity 0.2s;
  padding: 4px;
}

.edit-btn:hover {
  opacity: 1;
}

.edit-mode {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.goal-input {
  width: 100px;
  font-size: 0.85rem;
  padding: 6px 10px;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-weight: 700;
  color: #1e293b;
  outline: none;
}

.goal-input:focus {
  border-color: #f59e0b;
}

.save-btn {
  background: #f59e0b;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 6px 12px;
  font-size: 0.8rem;
  font-weight: 800;
  cursor: pointer;
  transition: transform 0.1s;
}

.save-btn:active {
  transform: scale(0.95);
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-grow: 1;
  color: #94a3b8;
  font-size: 0.9rem;
  font-weight: 600;
}
</style>
