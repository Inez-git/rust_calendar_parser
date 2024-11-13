fmt:
		cargo fmt

clippy:
		cargo clippy

test:
		cargo test

build:
		cargo build

run:
		cargo run parse event_examples.txt events_json.json

all: fmt clippy test build