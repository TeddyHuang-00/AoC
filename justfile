YEAR := ```
    fd -t d -E target "day*" . \
    | sed 's/.*year\([0-9]*\).*day\([0-9]*\).*/\1 \2/' \
    | sort -n \
    | tail -n 1 \
    | awk -v start="2014 00" '{ print } END { if ( NR == 0 ) { print start } }' \
    | awk '{ printf "%04d", $1 }'
```
NEXT_YEAR := ```
    fd -t d -E target "day*" . \
    | sed 's/.*year\([0-9]*\).*day\([0-9]*\).*/\1 \2/' \
    | sort -n \
    | tail -n 1 \
    | awk -v start="2014 00" '{ print } END { if ( NR == 0 ) { print start } }' \
    | awk '{ printf "%04d", $1 + 1 }'
```
DAY := ```
    fd -t d -E target "day*" . \
    | sed 's/.*year\([0-9]*\).*day\([0-9]*\).*/\1 \2/' \
    | sort -n \
    | tail -n 1 \
    | awk -v start="2014 00" '{ print } END { if ( NR == 0 ) { print start } }' \
    | awk '{ printf "%02d", $2 }'
```
NEXT_DAY := ```
    fd -t d -E target "day*" . \
    | sed 's/.*year\([0-9]*\).*day\([0-9]*\).*/\1 \2/' \
    | sort -n \
    | tail -n 1 \
    | awk -v start="2014 00" '{ print } END { if ( NR == 0 ) { print start } }' \
    | awk '{ printf "%02d", $2 + 1 }'
```

HAS_CHANGES := if path_exists(".jj") == "true" { 'test -n "$(jj diff --summary)"' } else if path_exists(".git") == "true" { 'test -n "$(git status --porcelain)"' } else { 'false' }
PRE_COMMIT := if path_exists(".jj") == "true" { ":" } else if path_exists(".git") == "true" { "git add -A" } else { ":" }
AT_COMMIT := if path_exists(".jj") == "true" { "jj desc -m" } else if path_exists(".git") == "true" { "git commit -m" } else { ":" }
POST_COMMIT := if path_exists(".jj") == "true" { "jj new" } else { ":" }

_default:
    @just --choose

_strip:
    fd -t f -E target -E template -g "*.rs" -x sed -i -E "\#^[[:space:]]*// (TEMPLATE):#d"

[doc("Format all code and sort Cargo.toml files")]
[group("housekeeping")]
format:
    cargo +nightly fmt --all
    cargo autoinherit --prefer-simple-dotted
    cargo sort --workspace > /dev/null 2>&1
    cargo sort-derives
    just --fmt --unstable

[doc("Run all checks including type checking, linting, and typo checking")]
[group("housekeeping")]
check: format
    typos **/*.rs
    cargo machete
    cargo check --all --workspace --quiet
    cargo clippy --workspace --quiet

[doc("Fix lint warnings automatically (safely)")]
[group("housekeeping")]
fix: _strip && format
    code=0; cargo machete --fix --with-metadata || code=$?; case $code in 0|1) exit 0;; *) exit 1;; esac
    cargo clippy --fix --allow-dirty --allow-staged --workspace --quiet

[doc("Run tests for a specific day's puzzle with example input")]
[group("puzzle")]
test Y=YEAR D=DAY:
    cargo test -p y{{ Y }}d{{ D }} "test_" --quiet -- --no-capture

[doc("Run the benchmark for a specific day's puzzle and record performance")]
[group("puzzle")]
bench Y=YEAR D=DAY:
    cargo test -r -p y{{ Y }}d{{ D }} "benchmark" --quiet -- --no-capture --test-threads 1

[doc("Run the solution for a specific day's puzzle with actual input")]
[group("puzzle")]
run Y=YEAR D=DAY:
    cargo run -r -p y{{ Y }}d{{ D }} --quiet

[doc("Check, test and then commit the solution to VCS")]
[group("puzzle")]
commit Y=YEAR D=DAY: (test Y D) check
    {{ HAS_CHANGES }} || (echo "No changes to commit"; exit 1)
    {{ PRE_COMMIT }}
    {{ AT_COMMIT }} "feat: Solution Year {{ Y }} Day {{ D }}"
    {{ POST_COMMIT }}

[doc("Create a new day's puzzle scaffold")]
[group("scaffold")]
new-day Y=YEAR D=NEXT_DAY: && format
    -rm -rf year{{ Y }}/day{{ D }}
    cargo new year{{ Y }}/day{{ D }} --name y{{ Y }}d{{ D }} --bin --vcs none
    touch year{{ Y }}/day{{ D }}/example.{in,out}
    touch year{{ Y }}/day{{ D }}/puzzle.in
    cp template/main.rs year{{ Y }}/day{{ D }}/src/main.rs
    cargo add -p y{{ Y }}d{{ D }} anyhow util

[doc("Create a new year's puzzle directory")]
[group("scaffold")]
new-year Y=NEXT_YEAR:
    -rm -rf year{{ Y }}
    mkdir year{{ Y }}
    @just new-day {{ Y }} 01

[doc("Summarize all solutions of a year")]
summary Y=YEAR: (bench Y "*")
    AOC_YEAR={{ Y }} cargo r -r --bin summary
