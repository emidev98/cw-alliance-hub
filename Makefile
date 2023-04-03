#!/usr/bin/make -f

###########
## Flows ##
###########

all: token alliance init delegate claim-rewards start-unbonding

.PHONY: all
#################################
## Alliance Hub Smart Contract ##
#################################

init: optimize-workspace
	bash ./scripts/init.sh

delegate:
	bash ./scripts/execute-contract/delegate.sh

start-unbonding:
	bash ./scripts/execute-contract/start-unbonding.sh

redeem-bond:
	bash ./scripts/execute-contract/redeem-bond.sh

claim-rewards:
	bash ./scripts/execute-contract/claim-rewards.sh

redelegate:
	bash ./scripts/execute-contract/redelegate.sh
	
build-migrate: optimize-workspace
	bash ./scripts/build-migrate.sh

optimize-workspace: 
	docker run --rm -v "$(shell pwd)":/code \
		--mount type=volume,source="$(shell basename $(shell pwd))_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/workspace-optimizer:0.12.13

.PHONY: init build-migrate delegate smart-contract-flow optimize-workspace

################################
## Native Alliance executions ##
################################

token:
	bash ./scripts/token-factory/create.sh

alliance:
	bash ./scripts/alliance/create.sh

.PHONY: token alliance