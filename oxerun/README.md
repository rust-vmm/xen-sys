[![Build status](https://gitlab.com/cardoe/oxerun/badges/master/pipeline.svg)](https://gitlab.com/cardoe/oxerun/commits/master)

# Example project using Rust for Xen unikernels

This is an example build of Rust building a full unikernel for Xen.

## Building

```shell
# cargo build --target x86_64-xen-pv.json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem
```

And to generate assember files in target/x86_64-xen-pv/{release|debug}/deps/

```shell
# RUSTFLAGS="--emit asm -C llvm-args=-x86-asm-syntax=intel" cargo build --target x86_64-xen-pv.json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem
```
