# Hello World

1. Download from [here](https://prometheus.io/download/) the prometheus binary, or use a docker container, etc that contains is.
2. Copy the `prometheus.yml` in your home directory of the target environment
3. Open the CLI and run prometheus, make sure to pass as an argument the correct configuration `--config.file="prometheus.yml"`.
4. Inspect `http://localhost:9090` and the prometheus UI should be available
5. Make sure that the targets are reachable