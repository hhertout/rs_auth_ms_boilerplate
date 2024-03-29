# Create migration file
migration:
	@echo "Creating migration file"
	@sqlx migrate add $(filter-out $@,$(MAKECMDGOALS))

test:
	@echo "Running tests"
	@cargo test --all -- --test-threads=1

dc-up:
	@echo "Starting docker app"
	@docker compose up --build

dc-down:
	@echo "Shutdown docker app"
	@docker compose down -v

dc-reset:
	@eco "Cleaning the docker app"
	@docker compose down -v && docker compose up --build