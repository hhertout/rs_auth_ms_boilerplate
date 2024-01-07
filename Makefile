# Create migration file
migration:
	@echo "Creating migration file"
	@sqlx migrate add $(filter-out $@,$(MAKECMDGOALS))