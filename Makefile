#!/usr/bin/make -f


#################################
## Alliance Hub Smart Contract ##
#################################

init:
	bash ./scripts/init.sh

execute:
	bash ./scripts/execute/delegate.sh

build-migrate:
	bash ./scripts/build-migrate.sh

.PHONY: init build-migrate execute

################################
## Native Alliance executions ##
################################

token:
	bash ./scripts/token-factory/create.sh

alliance:
	bash ./scripts/alliance/create.sh

.PHONY: token alliance