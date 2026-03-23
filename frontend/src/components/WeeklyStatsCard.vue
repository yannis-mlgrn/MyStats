<script setup>
defineProps({
  stats: {
    type: Object,
    required: true
  }
})
</script>

<template>
  <div class="dashboard-card streaks-card">
    <div class="card-header">
      <h3 class="card-title">Kilomètres de la semaine</h3>
    </div>

    <div class="streaks-body">
      <div class="streak-block course-bg">
        <div class="streak-line course"></div>
        <div class="streak-val">{{ stats.running_km.toFixed(1) }}</div>
        <div class="streak-label">Course (km)</div>
      </div>

      <div class="streak-block velo-bg">
        <div class="streak-line velo"></div>
        <div class="streak-val">{{ stats.cycling_km.toFixed(1) }}</div>
        <div class="streak-label">Vélo (km)</div>
      </div>

      <div class="streak-block natation-bg">
        <div class="streak-line natation"></div>
        <div class="streak-val">{{ (stats.swimming_m / 1000).toFixed(1) }}</div>
        <div class="streak-label">Nage (km)</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dashboard-card {
  background: var(--bg-card, #fff);
  border: 1px solid #f3f4f6;
  border-radius: 16px;
  padding: 1.5rem;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.05);
  width: 100%;
  height: 100%;
  transition: transform 0.2s ease;
}

.dashboard-card:hover {
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  margin-bottom: 1.5rem;
}

.card-title {
  font-size: 1.1rem;
  font-weight: 800;
  color: #111827;
  margin: 0;
}

/* STREAKS CARD BODY */
.streaks-body {
  display: flex;
  gap: 1rem;
  height: calc(100% - 3rem);
}

.streak-block {
  text-align: center;
  flex: 1;
  padding: 1.5rem 0.75rem;
  border-radius: 12px;
  background: #fff;
  border: 1px solid #f9fafb;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.course-bg { background-color: #fffaf0; }
.velo-bg { background-color: #f0f7ff; }
.natation-bg { background-color: #f0fdff; }

.streak-line {
  height: 4px;
  margin-bottom: 1rem;
  width: 40px;
  margin-left: auto;
  margin-right: auto;
  border-radius: 2px;
}

.streak-line.course { background: #f59e0b; }
.streak-line.velo { background: #3b82f6; }
.streak-line.natation { background: #06b6d4; }

.streak-val {
  font-size: 2.2rem;
  font-weight: 800;
  color: #1f2937;
  line-height: 1;
  margin-bottom: 0.5rem;
}

.streak-label {
  font-size: 0.7rem;
  color: #6b7280;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
</style>
