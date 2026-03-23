# Initialiser et démarrer la base de données PostgreSQL locale
db-start:
	@echo "🐘 Démarrage de la base de données..."
	@if [ ! -d ".pgdata" ]; then \
		echo "Initialisation du cluster PostgreSQL..."; \
		initdb -U postgres -D .pgdata --no-locale --encoding=UTF8; \
	fi
	@pg_ctl -D .pgdata -l .pgdata/pg.log -o "-p 5432 -k $PWD/.pgdata" start || true
	@createdb -p 5432 -h localhost -U postgres mystats 2>/dev/null || true

# Arrêter la base de données PostgreSQL
db-stop:
	@echo "🛑 Arrêt de la base de données..."
	@pg_ctl -D .pgdata stop || true

# Lancer le Frontend, le Backend et la DB en même temps
run: db-start
	@echo "🚀 Démarrage de MyStats (Backend & Frontend & DB)..."
	@trap 'just db-stop; kill 0' SIGINT EXIT; \
	(cd backend && cargo run) & \
	(cd frontend && npm run dev) & \
	wait

# Lancer uniquement le Backend Rust
run-back: db-start
	@trap 'just db-stop; kill 0' SIGINT EXIT; \
	cd backend && cargo run

# Lancer uniquement le Frontend Vue.js
run-front:
	cd frontend && npm run dev

# Lancer tous les linters (Front & Back)
lint: lint-front lint-back

# Linter le Frontend Vue.js
lint-front:
	cd frontend && npm run lint

# Linter le Backend Rust (Offline mode if .sqlx exists)
lint-back:
	cd backend && SQLX_OFFLINE=true cargo clippy -- -D warnings

# Préparer les métadonnées SQLx pour le linting offline
prepare-back: db-start
	cd backend && cargo sqlx prepare

# Formater tout le code (Front & Back)
format: format-front format-back

# Formater le Frontend Vue.js
format-front:
	cd frontend && npm run format

# Formater le Backend Rust
format-back:
	cd backend && cargo fmt

# Lancer toutes les vérifications DevSecOps (Audit & Secrets)
check-security:
	@echo "🔒 Lancement de l'audit"
	@echo "[1/3] Vérification des secrets (Gitleaks)"
	gitleaks detect --source . -v
	@echo "[2/3] Audit des dépendances Rust"
	cd backend && cargo audit
	@echo "[3/3] Analyse des licences et dépendances bannies"
	cd backend && cargo deny check
	@echo "✅ Toutes les vérifications de sécurité sont passées avec succès !"
