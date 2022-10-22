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

The two origin server services will be available at `localhost:30001` and `localhost:30002` while `frontman` will be available at `localhost:30000`
