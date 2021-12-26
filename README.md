oxerun
======

Project Objective
-----------------

The goal of this project is to provide Rust native Xen hypercall
and interface support. It is aimed to be divided into 3 different
main components components:

- [xen](/xen) - Rust idiomatic Xen interfaces
- [xen-sys](/xen-sys) - Xen native bindings that should be unsafe
- [oxerun](/oxerun) - Unikernel generator for Xen

The goal is that you can compile the same code into
- a Linux binary that can execute hypercalls from dom0/domU
- a unikernel when packaged with oxerun

oxerun
------

oxerun's goal is to be in the same thread as the
[bootloader](https://github.com/rust-osdev/bootloader) from
[phil-opp](https://github.com/phil-opp) and the
[rust-osdev](https://github.com/rust-osdev) project.
Eventually the user should be able run something similar to:

```
cargo oxerun --target <x86_64/aarch64>-xen-<hvm/pv/pvh>
```

in the source directory of your project. Your project will
receive the kernel boot arguments from the `cmdline` parameters
as argv.

Currently however oxerun is more of an example than the actual
tool. Eventually this code should get moved into an examples
directory within the repo.

xen
---

This crate aims to provide idomatic Rust interfaces for interacting
with the hypervisor.


