PROTO=xcb-proto-1.13
REF_OUT=target/code-gen-ref
CMP_OUT=target/code-gen-cmp

help:
	@echo "If you want to build x11rb: Just ignore this file. You do not need it."
	@echo
	@echo "This Makefile helps in development of x11rb. It can be used to evaluate"
	@echo "changes to the code generator. With this Makefile, a copy of the generated"
	@echo "code can be saved. Later (after changing the code generator), a diff can"
	@echo "can be generated to see exactly in what way the generated code changed."
	@echo
	@echo "Usage:"
	@echo
	@echo "Use 'make reference' to save the current state as the reference."
	@echo
	@echo "Later, 'make check' compares the saved reference with the new state of"
	@echo "the code generator."

ref: reference

cmp: check

reference:
	mkdir -p "$(REF_OUT)"
	python rs_code_generator.py -p "$(PROTO)" -i "$(PROTO)/src" -o "$(REF_OUT)" generated

check:
	mkdir -p "$(CMP_OUT)"
	python rs_code_generator.py -p "$(PROTO)" -i "$(PROTO)/src" -o "$(CMP_OUT)" generated
	diff -Nurp "$(REF_OUT)" "$(CMP_OUT)"


.PHONE: help ref reference check cmp
