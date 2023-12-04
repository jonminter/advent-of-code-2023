day-01:
	cat inputs/day-01.txt | cargo run --bin day-01

day-02:
	cat inputs/day-02.txt | cargo run --bin day-02

dev-day-02:
	cd crates/day-02 && cargo watch -x check -x test

day-03:
	cat inputs/day-03.txt | cargo run --bin day-03

dev-day-03:
	cd crates/day-03 && cargo watch -x check -x test

day-04:
	cat inputs/day-04.txt | cargo run --bin day-04

dev-day-04:
	cd crates/day-04 && cargo watch -x check -x test