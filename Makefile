build-dev:
	docker compose -f docker-compose.dev.yaml build

start-dev:
	docker compose -f docker-compose.dev.yaml up --build

start:
	docker compose up --build