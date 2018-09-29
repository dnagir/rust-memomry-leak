## Potential memory leak repro

I was running a small project on hyper.sh and noticed my containers died.
This was happening because the memory kept growing with use.

So my observations:

- memory grows on hyper.sh
- memory grows when compiled for alpine Docker image
- memory does not seem to grow when compiled on MacOS

## Benchmarks

- run alpine - `script/stress-alpine`
- run locally - `script/stress-local`


## Environment

- Rust 1.28 stable
- Docker 18.06.0-ce-mac69
- need `pgrep` to run local test

## My output - alpine

The memory started from aroun 12MB and went up to ~54MB.

```
================= Iteration 1
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT     MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.23%               12.05MiB / 1.952GiB   0.60%               3.02MB / 13kB       0B / 0B             28
================= Iteration 2
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT     MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.18%               19.28MiB / 1.952GiB   0.96%               6.03MB / 26.9kB     0B / 0B             28
================= Iteration 3
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT     MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.18%               28.46MiB / 1.952GiB   1.42%               9.04MB / 35kB       0B / 0B             28
================= Iteration 4
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT     MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.16%               36.58MiB / 1.952GiB   1.83%               12.1MB / 44.6kB     0B / 0B             28

...

================= Iteration 47
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT     MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.13%               52.64MiB / 1.952GiB   2.63%               142MB / 610kB       0B / 0B             28
================= Iteration 48
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT     MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.17%               52.64MiB / 1.952GiB   2.63%               145MB / 621kB       0B / 0B             28
================= Iteration 49
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT    MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.14%               54.3MiB / 1.952GiB   2.72%               148MB / 634kB       0B / 0B             28
================= Iteration 50
CONTAINER ID        NAME                CPU %               MEM USAGE / LIMIT    MEM %               NET I/O             BLOCK I/O           PIDS
60588c986c60        pedantic_raman      0.15%               54.2MiB / 1.952GiB   2.71%               151MB / 645kB       0B / 0B             28
```

## My output - local

The memory started from aroun 12MB and went up to ~19MB.

```
================= Iteration 1
Memory (KB):  11504
================= Iteration 2
Memory (KB):  12760
================= Iteration 3
Memory (KB):  14012
================= Iteration 4
Memory (KB):  15260

...

================= Iteration 47
Memory (KB):  17916
================= Iteration 48
Memory (KB):  19200
================= Iteration 49
Memory (KB):  17548
================= Iteration 50
Memory (KB):  19380
```
