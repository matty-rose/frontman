# Frontman

## Demos

See the [`demos`](./demos) directory.

## TODO
- [x] Read/parse proxy config from file?
- [x] Server CLI args (config file path, listening port etc)
- [ ] Load balance requests to 1 or more origin servers (of the same backend), supporting IP/DNS
- [ ] Anyhow for result?
- [ ] proper error responses
- [x] docker-compose setup/example
- [x] local k8s setup/example
- [ ] Persist specific origin server (by name?) for requests through cookies (if enabled in config?)
- [ ] Load/perf testing
- [ ] add origin groups (ie load balance to two different services/backends, each with their own group of 1 or more servers) with path based routing
