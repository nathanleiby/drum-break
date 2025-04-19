alias cc := code_coverage
alias pc := pre_commit
alias t := test_all
alias tw := test_watch_all

lint:
    cargo clippy --all --all-features --tests -- -D warnings

lint_watch:
    git ls-files | entr just lint

lint_fix:
    cargo clippy --all --all-features --tests --fix

build:
    cargo build

test_all:
    cargo test

test_watch_all:
    git ls-files | entr cargo test

test TEST:
    cargo test {{TEST}}

test_watch TEST:
    git ls-files | entr cargo test {{TEST}}

code_coverage:
    cargo tarpaulin -o html && open tarpaulin-report.html

pre_commit: lint test_all build
