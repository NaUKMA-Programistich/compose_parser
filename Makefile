format:
	cargo fmt && cargo clippy

PUBLISH_MESSAGE := "update"

publish:
	git add . && git commit -m $(PUBLISH_MESSAGE) && git push origin && cargo publish

test:
	cargo test

docs:
	cargo doc --no-deps --open