# Run all linters
lint: lint-front lint-back

# Lint the Vue.js frontend
lint-front:
	cd frontend && npm run lint

# Lint the Rust backend
lint-back:
	cd backend && cargo clippy -- -D warnings

# Format all code
format: format-front format-back

# Format the Vue.js frontend
format-front:
	cd frontend && npm run format

# Format the Rust backend
format-back:
	cd backend && cargo fmt
