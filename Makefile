#!/usr/bin/make -f

build-deploy-instantiate:
	bash ./scripts/build-deploy-instantiate.sh

execute:
	bash ./scripts/execute/delegate.sh

build-migrate:
	bash ./scripts/build-migrate.sh

.PHONY: build-deploy-instantiate build-migrate