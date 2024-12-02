# Advent of code 2024

## Getting started
This project uses the [Just crate](https://github.com/casey/just) as a command runner.
Add Just to your system by running the following...

``` shell 
    cargo install just
```

During development, run the following to watch for code changes and test a specific day and part, for example...

```shell
    just dev day-1 part1
```

## Development
This project uses workspaces. 
When starting a new challenge, for example day 5, run the following from the 
projects root directory... 
```shell 
    cargo new day-5
```

Create a separate binary for each part by adding a `/bin` folder to the `/src` directory, 
then inserting `part1.rs` and eventually `part2.rs`.