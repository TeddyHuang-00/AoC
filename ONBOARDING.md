# Onboarding guide

## Setup

You are more than welcome to use this as a template for your own Advent of Code solutions. To get started, follow these steps:

1. **Make this your repository**
   - Either fork or clone this repository to your local machine.
   - Delete the `yearXXXX` directories to start fresh.
   - Replace current license files with your own if necessary.
2. **Set up your environment**
   - Ensure you have Rust and Cargo installed on your machine.
     - You can install Rust using [rustup](https://rustup.rs/).
     - Make sure to install the nightly toolchain as well as some tasks may require it:
       ```sh
       rustup install nightly
       ```
     - Make sure to also include the necessary components:
       ```sh
       rustup component add rustfmt clippy
       ```
   - Install `just` for task automation if you haven't already.
   - Command line tools like `fd`, `sed`, and `awk` are also required for some tasks in the `justfile`.
   - Extra cargo tools used in the `justfile`:
     - `cargo-autoinherit` (for managing dependencies)
     - `cargo-sort` (for sorting Cargo.toml)
     - `cargo-sort-derives` (for sorting derive macros)

     All can be installed via Cargo or `cargo-binstall`:

     ```sh
     cargo install cargo-autoinherit cargo-sort cargo-sort-derives
     # or
     cargo binstall cargo-autoinherit cargo-sort cargo-sort-derives
     # or your preferred method, e.g., system package managers
     ```

## Usage

### Overview

All commands are run via `just`, a command runner. You can see all available commands by running:

```sh
# list and select interactively (requires `fzf`):
just
# or statically list them with:
just --list
```

### Creating a New Year

To create a new year's project structure, run:

```sh
just new-year
```

This will create a new directory for the next year (depending on the current non-empty year folder; if empty, defaults to 2015) and the first day's project structure.

You can also specify a particular year, for example, 2026:

```sh
just new-year 2026
```

### Creating a New Day

To create a new day's project structure, run:

```sh
just new-day
```

This will determine the latest year you are working on, create a new directory for the next day, set up input files, and initialize a new Rust binary project.

If you want to specify a particular day number, for example, day 5, you need to also specify the year:

```sh
just new-day 2015 05
# Note: Use two digits for the day number.
```

The input and output files (`*.{in,out}`) will be created in the same directory, you should fill them with the appropriate data for that day's challenge:

- `example.in` - Example input data provided in the challenge description.
- `example.out` - Expected output for the example input. One line per part of the challenge (part 1 and part 2). Leave lines blank if the expected output is not provided.
- `puzzle.in` - Puzzle input data provided to you only. You should not share this file with anyone else.

### Running the Solution

To test your solution on the example input for the latest year and day, run:

```sh
just test
```

To run your solution on the actual input for the latest year and day, run:

```sh
just run
```

Both commands will automatically determine the latest year and day based on the existing directories. You may also specify a particular year and day by providing the them as arguments:

```sh
just test 2025 05
just run 2025 05
# Note: Use two digits for the day number. Same as in `just new`.
```

> Hint: For testing specifically, you can also run the tests for the whole year or the whole project:
>
> ```sh
> just test 2025 "*"
> # Note: Use `*` to match tests for all days in the specified year.
> just test "*" "*"
> # Note: Use `*` to match tests for all years and all days.
> ```

### Housekeeping Chores

To format the code and sort dependencies, run:

```sh
just format
```

To lint the code, run:

```sh
just check
```

To apply safe fixes suggested by clippy, run:

```sh
just fix
```
