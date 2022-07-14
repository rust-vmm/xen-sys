
.cargo/config.toml

content:
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

Compilation for aarch64:

cargo build --target=aarch64-unknown-linux-gnu
