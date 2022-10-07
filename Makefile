SHELL=/bin/bash
.DEFAULT_GOAL=help

APP=frontman

.PHONY: help
help:
	@grep -E \
		'^.PHONY: .*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ".PHONY: |## "}; {printf "\033[36m%-16s\033[0m \t\t%s\n", $$2, $$3}'

.PHONY: frontman/docker ## Builds the frontman docker image
frontman/docker:
	docker build . -f docker/frontman.Dockerfile -t frontman

.PHONY: origin/docker ## Builds the demo origin server docker image
origin/docker:
	docker build demos/origin -f docker/origin.Dockerfile -t origin

.PHONY: cluster/up ## Creates a local kubernetes cluster with kind
cluster/up:
	kind create cluster --name $(APP) --config demos/k8s/kind.yaml

.PHONY: cluster/load-images ## Loads locally built docker images to the running kind cluster
cluster/load-images:
	kind load docker-image frontman --name frontman
	kind load docker-image origin --name frontman

.PHONY: cluster/deploy ## Deploys frontman and origin servers to k8s cluster
cluster/deploy:
	kubectl apply -f demos/k8s/origin-deployment.yaml
	kubectl apply -f demos/k8s/frontman-deployment.yaml

.PHONY: cluster/down ## Deletes the local kubernetes cluster
cluster/down:
	kind delete cluster --name $(APP)

