# DCPM

**DCPM** (**D**ocker **C**ontainer **P**ID **M**apper) is a utility to determine mapping between PID in host namespace and docker container namespace. Only Docker running natively on linux host OS is supported.

Works by running `docker top {container}` to grab host PIDs and looks at `NSpid` entry in `/proc/{pid}/status` file.

# Installing

Can be compiled from source
```shell
git clone https://github.com/mee7ya/dcpm.git
cd dcpm
cargo build --release
```

# Example

```shell
$ dcpm my-container
HOST_PID CONTAINER_PID COMMAND
    6588 1             postgres
    6681 27            postgres: checkpointer
    6682 28            postgres: background writer
    6684 30            postgres: walwriter
    6685 31            postgres: autovacuum launcher
    6686 32            postgres: logical replication launcher
```