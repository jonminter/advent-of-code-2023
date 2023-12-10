download-input:
	curl -o inputs/day-$$(printf "%02d" $(day)).txt https://adventofcode.com/2023/day/$(day)/input

run:
	cat inputs/day-$$(printf "%02d" $(day)).txt | cargo run --bin day-$$(printf "%02d" $(day))

dev:
	cd crates/day-$$(printf "%02d" $(day)) && cargo watch -x check -x test -s  "cat ../../inputs/day-$$(printf "%02d" $(day)).txt | cargo run --bin day-$$(printf "%02d" $(day))"