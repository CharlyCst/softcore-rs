TEST_ARCHS = basic basic_alt trap types mret csr wfi
TEST_ARCH_RUST = $(addsuffix /src/arch.rs, $(addprefix tests/, $(TEST_ARCHS)))

.PHONY: tests test test2 clean

install: 
	sudo apt-get install opam
	sudo apt-get install z3

	opam init
	eval $(opam env)

	opam install dune
	opam install libsail.0.17.1
	opam install sail.0.17.1
	
all: build

tests: $(TEST_ARCH_RUST)

build:
	dune build --release

# Translate the test Sail architectures to Rust
tests/%/src/arch.rs: sail_arch/%.sail build
	@echo "Processing $< to $@"
	./sail_to_rust $< -o $@

csr: build
	./sail_to_rust sail_arch/csr.sail -o out.rs

mret: build
	./sail_to_rust sail_arch/mret.sail -o out.rs

basic_types: build
	./sail_to_rust sail_arch/basic_types.sail -o out.rs

trap: build
	./sail_to_rust sail_arch/arch_trap.sail -o out.rs

wfi: build
	./sail_to_rust sail_arch/wfi.sail -o out.rs

riscv: build
	./sail_to_rust $(RISCV_SAIL_SRCS) ../miralis-sail-riscv/model/main.sail -o out.rs

generate: build
	./sail_to_rust sail_arch/mret.sail -o ./rust_arch/mret/src/sail.rs

generate-riscv: build
	./sail_to_rust $(RISCV_SAIL_SRCS) ../miralis-sail-riscv/model/main.sail -o ./src/lib.rs
	python3 manual_fixes.py

riscv-c:  
	sail -c $(RISCV_SAIL_SRCS) 1> riscv_c.c 

verify: build
	cd rust_arch/mret && cargo kani 


decoder: build
	./sail_to_rust sail_arch/decoder.sail -o ./src/lib.rs
	python3 manual_fixes.py

clean:
	-dune clean
