<template>
  <header class="top-nav">
    <div class="top-icons">
      <!-- <div class="profile-pic"></div> -->
    </div>
  </header>
</template>

<style scoped>
.top-nav {
  height: 70px;
  background: var(--bg-card, #fff);
  border-bottom: 1px solid var(--border-light, #e5e7eb);
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 0 2rem;
}

.top-icons {
  display: flex;
  align-items: center;
}

/* TODO: Add multi account connection */
/* .profile-pic {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: var(--color-brand, #2563eb);
  background-image: url('https://ui-avatars.com/api/?name=Yannis&background=2563eb&color=fff');
  background-size: cover;
  border: 2px solid white;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
} */
</style>
