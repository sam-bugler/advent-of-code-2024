set shell := ["powershell.exe", "-c"]

default:
    just --list

#use `just dev day-1 part1` to work on a specific day
dev day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"

test day part:
    cargo test -p {{day}} {{part}} -- --nocapture

lint day:
    cargo clippy -p {{day}}
