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
  display: flex;
  flex-direction: column;
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
  flex: 1; /* Take remaining height */
  min-height: 0;
}

.streak-block {
  text-align: center;
  flex: 1;
  padding: 1.5rem 0.75rem;
  border-radius: 16px; /* Slightly more rounded */
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid transparent;
}

.streak-block:hover {
  transform: translateY(-4px);
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.05);
}

.course-bg {
  background-color: rgba(245, 158, 11, 0.1);
  border-color: rgba(245, 158, 11, 0.2);
}
.velo-bg {
  background-color: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.2);
}
.natation-bg {
  background-color: rgba(6, 182, 212, 0.1);
  border-color: rgba(6, 182, 212, 0.2);
}

.streak-line {
  height: 6px;
  margin-bottom: 1rem;
  width: 32px;
  border-radius: 3px;
}

.streak-line.course {
  background: #f59e0b;
  box-shadow: 0 2px 4px rgba(245, 158, 11, 0.2);
}
.streak-line.velo {
  background: #3b82f6;
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.2);
}
.streak-line.natation {
  background: #06b6d4;
  box-shadow: 0 2px 4px rgba(6, 182, 212, 0.2);
}

.streak-val {
  font-size: 2.4rem;
  font-weight: 900;
  color: #0f172a;
  line-height: 1;
  margin-bottom: 0.5rem;
}

.streak-label {
  font-size: 0.75rem;
  color: #475569;
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

@media (max-width: 640px) {
  .streaks-body {
    flex-direction: column;
  }

  .streak-block {
    padding: 1rem;
  }

  .streak-val {
    font-size: 1.8rem;
  }
}
</style>
