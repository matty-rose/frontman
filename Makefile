frontman/docker:
	docker build . -f docker/frontman.Dockerfile -t frontman

origin/docker:
	docker build demos/origin -f docker/origin.Dockerfile -t origin

