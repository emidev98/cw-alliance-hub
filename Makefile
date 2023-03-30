#!/usr/bin/make -f

###########
## Flows ##
###########

all: token alliance init delegate build-migrate

.PHONY: all
#################################
## Alliance Hub Smart Contract ##
#################################

init: contract-optimize
	bash ./scripts/init.sh

delegate:
	bash ./scripts/execute-contract/delegate.sh

claim-rewards:
	bash ./scripts/execute-contract/claim-rewards.sh

build-migrate: contract-optimize
	bash ./scripts/build-migrate.sh

contract-all: init delegate claim-rewards

contract-optimize: 
	docker run --rm -v "$(shell pwd)":/code \
		--mount type=volume,source="$(shell basename $(shell pwd))_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.12.12

.PHONY: init build-migrate delegate smart-contract-flow contract-optimize

################################
## Native Alliance executions ##
################################

token:
	bash ./scripts/token-factory/create.sh

alliance:
	bash ./scripts/alliance/create.sh

.PHONY: token alliance