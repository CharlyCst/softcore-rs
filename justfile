# Print the list of commands
help:
	@just --list --unsorted

# Run the tests
test:
    make -B tests
    cargo test
