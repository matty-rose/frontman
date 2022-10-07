# K8s

## Setup
From the root of the repository build the docker images with
```shell
make frontman/docker
make origin/docker
```

Then create a local kind cluster and deploy the workloads with
```shell
make cluster/up
make cluster/load-images
make cluster/deploy
```
