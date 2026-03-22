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
      <h3>Kilomètres de la semaine</h3>
    </div>

    <div class="streaks-body">
      <div class="streak-block">
        <div class="streak-line brand"></div>
        <div class="streak-val">{{ stats.running_km.toFixed(1) }}</div>
        <div class="streak-label">course (km)</div>
      </div>
      <div class="streak-divider"></div>
      <div class="streak-block">
        <div class="streak-line brand"></div>
        <div class="streak-val">{{ stats.cycling_km.toFixed(1) }}</div>
        <div class="streak-label">vélo (km)</div>
      </div>
      <div class="streak-divider"></div>
      <div class="streak-block">
        <div class="streak-line brand"></div>
        <div class="streak-val">{{ (stats.swimming_m / 1000).toFixed(1) }}</div>
        <div class="streak-label">natation (km)</div>
      </div>
    </div>

    <div class="streaks-footer">
      Données récupérées depuis l'API locale
    </div>
  </div>
</template>

<style scoped>
.dashboard-card {
  background: var(--bg-card, #fff);
  border: 1px solid #d1d5db; /* Contrasting light border */
  border-radius: 8px; /* Slightly more rigid corners */
  padding: 2rem; /* Reduced padding from 2.5rem */
  box-shadow: 0 10px 15px -3px rgba(0,0,0,0.08), 0 4px 6px -2px rgba(0,0,0,0.04);
  max-width: 650px; /* Reduced overall max width */
  margin: 0 auto;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 2rem; /* Reduced from 3rem */
}

.card-header h3 {
  font-size: 1.15rem; /* Reduced from 1.35rem */
  font-weight: 600;
  color: var(--text-main, #111827);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* STREAKS CARD BODY */
.streaks-body {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.streak-block {
  text-align: center;
  flex: 1;
  padding: 0 1rem; /* Reduced padding */
}

.streak-line.brand {
  height: 3px;
  background: var(--color-brand, #2563eb);
  margin-bottom: 1rem; /* Reduced margin */
  width: 100%;
  border-radius: 2px;
}

.streak-val {
  font-size: 2.5rem; /* Reduced specifically from 3.5rem as requested 'moins gros' */
  font-weight: 800;
  color: var(--text-main, #111827);
  line-height: 1;
  margin-bottom: 0.5rem;
  letter-spacing: -1px;
}

.streak-label {
  font-size: 0.85rem;
  color: var(--text-muted, #6b7280);
  font-weight: 500;
}

.streak-divider {
  width: 1px;
  height: 60px; /* Reduced from 80px */
  background: var(--border-light, #e5e7eb);
}

.streaks-footer {
  margin-top: 2.5rem; /* Reduced from 3.5rem */
  text-align: center;
  font-size: 0.8rem;
  color: #9ca3af;
}
</style>
