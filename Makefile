PROTO=xcb-proto-1.14-1-g2b3559c
OUT=src/generated
PYTHON=python

generate:
	mkdir -p "$(OUT)"
	$(PYTHON) rs_code_generator.py -p "$(PROTO)" -i "$(PROTO)/src" -o "$(OUT)"

.PHONY: generate
