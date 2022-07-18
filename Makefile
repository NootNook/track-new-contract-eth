all : update build

update :; cargo update
build :; cargo build
check:; cargo check
run :; cargo run

release :; cargo build -r

.PHONY: run build