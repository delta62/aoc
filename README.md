# aoc

Advent of code 2022

See other branches for past years!

## Usage

This project uses [Leiningen](https://leiningen.org/) to install dependencies
and run. It uses a helper library,
[aoc-runner](https://github.com/delta62/aoc-runner), to automate some of the
normal Advent of Code biolerplate out. Each day contains functions `part1`,
`part2`, and optionally a `parse` function which are invoked automatically.

By default, the latest day there is a package for is run.

```sh
lein run -- --help
-d, --day DAY  Run a specific day
-a, --all      Run all days
-h, --help     Show help

lein test
lein run
lein run -- --day 1
```
