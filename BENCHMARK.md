# Benchmarking

In order to validate that rust is indeed _fast_ I conducted a quick benchmark. It was done on a single dual-core linux machine with 8gb of RAM and an SSD. Not super stable. But gives a decent comparison of speeds.

**Options tested:**

- `git-local-prune`
- `github hub`: [sync](https://hub.github.com/hub-sync.1.html) command will cleanup old branches among other tasks
- `bash script`: [this](https://stackoverflow.com/a/17029936) stack overflow answer gives a nifty script I'm sure thousands of devs are using.

Each option is tested against the benchmark [script](test/end-to-end/run_benchmark.sh). This creates a variable number of branches for the tool to cleanup. The time taken is recorded.

**Results:**

Note: all results are in milliseconds

| Tool            | 10 Branches | 100 Branches | 1,000 Branches |
|-----------------|-------------|--------------|----------------|
| git-local-prune | 6ms         | 10ms         | 46ms           |
| hub sync        | 80ms        | 485ms        | 16540ms        |
| bash script     | 10ms        | 70ms         | 3550ms         |
