PROTO=xcbproto-1.13-6-ge79f6b0
OUT=src/generated
PYTHON=python

generate:
	mkdir -p "$(OUT)"
	$(PYTHON) rs_code_generator.py -p "$(PROTO)" -i "$(PROTO)/src" -o "$(OUT)"

.PHONY: generate
