PROTO=xcb-proto-1.14-1-g2b3559c
OUT=src/protocol

generate:
	mkdir -p "$(OUT)"
	cargo run -p x11rb-generator -- -i "$(PROTO)/src" -o "$(OUT)"

.PHONY: generate
