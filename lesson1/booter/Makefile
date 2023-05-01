# Allows the usage of unstable features in stable compilers.
export RUSTC_BOOTSTRAP := 1

OBJCOPY ?= rust-objcopy --binary-architecture=riscv64

OUT_DIR := target/riscv64gc-unknown-none-elf/release/
OUT_ELF := $(OUT_DIR)/booter
OUT_BIN := $(OUT_DIR)/booter.bin

all: build

build: $(OUT_BIN)

$(OUT_BIN): _cargo_build $(OUT_ELF)
	$(OBJCOPY) $(OUT_ELF) --strip-all -O binary $@

_cargo_build:
	cargo rustc -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem --target riscv64gc-unknown-none-elf --release --manifest-path Cargo.toml -- -Clink-arg=-T./linker_riscv64.lds

run: justrun
	@:

justrun:
	qemu-system-riscv64 -m 128M -smp 1 -machine virt -bios default -kernel $(OUT_DIR)/booter.bin -nographic

clean:
	cargo clean

.PHONY: all build clean run justrun _cargo_build
