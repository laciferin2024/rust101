set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set allow-duplicate-recipe
set positional-arguments
#set dotenv-load
set dotenv-filename := ".env"
set export

setup:
    cargo install cargo-script


run *ARGS:
    run-cargo-script {{ARGS}}

default: run

rust: run