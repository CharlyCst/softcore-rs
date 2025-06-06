TEST_ARCHS = basic basic_alt trap types mret csr wfi config hoisting
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

# Build the compiler
build:
	dune build --release

# Translate the test Sail architectures to Rust
tests/%/src/arch.rs: sail_arch/%.sail build
	@echo "Processing $< to $@"
	./sail_to_rust $< -o $@

clean:
	-dune clean
