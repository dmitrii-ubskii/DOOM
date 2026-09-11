.PHONY: run

run:
	cargo build
	Xephyr :1 -screen 960x600x8 2>/dev/null &
	bash -c "\
		trap 'killall Xephyr' EXIT; \
		DISPLAY=:1 cargo run -- -file linuxdoom-1.10/linux/devdata/doomu.wad -3 \
	"
	killall Xephyr
