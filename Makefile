day-01:
	cat inputs/day-01.txt | cargo run --bin day-01

day-02:
	cat inputs/day-02.txt | cargo run --bin day-02

dev-day-02:
	cd crates/day-02 && cargo watch -x check -x test