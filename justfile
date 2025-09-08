riscv-version := "b6f7b1df64157e6b9d250e552842f963394bc2ba"

# Print the list of commands
help:
	@just --list --unsorted

# Run the tests
test:
    make -B tests
    cargo test

# Format code
fmt:
    dune fmt
    cargo fmt

# Build the rv64 model
rv64:
    make build
    make -C sail_models/sail-riscv/build generated_rust_rv64d

armv9a:
    make build
    make -C sail_models/sail-arm/arm-v9.4-a gen_rs

# Download Sail models
download_models:
    # RISC-V
    rm -rf sail_models/sail-riscv
    git clone https://github.com/riscv/sail-riscv.git sail_models/sail-riscv
    cd ./sail_models/sail-riscv && git checkout {{riscv-version}}
    cd ./sail_models/sail-riscv && git apply ../riscv.patch
    cmake -S ./sail_models/sail-riscv -B ./sail_models/sail-riscv/build -DCMAKE_BUILD_TYPE=RelWithDebInfo

    # Arm
    git clone https://github.com/rems-project/sail-arm.git sail_models/sail-arm
    cd ./sail_models/sail-arm && git apply ../arm.patch

