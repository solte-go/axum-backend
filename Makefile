.PHONY: start-local-dev
start-local-dev:
	docker compose -f ./deployment/docker-compose.yaml -f ./deployment/docker-compose-dev.yaml up -d

.PHONY: stop-local-dev
stop-local-dev:
	docker compose -f ./deployment/docker-compose.yaml -f ./deployment/docker-compose-dev.yaml down

.PHONY: teardown-local-dev
teardown-local-dev:
	docker compose -f ./deployment/docker-compose.yaml -f ./deployment/docker-compose-dev.yaml down -v