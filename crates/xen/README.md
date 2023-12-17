# Goal

This project aims to provide some idiomatic Rust interfaces
to writing Rust based kernel level (or unikernels) for the Xen
Hypervisor. It purposefully does not wrap libxc or libxl and
should not ever. In a way it can be like a Rust native kernel
abstraction of the Xen hypercall / event channel / page sharing
interfaces. There's a possibility of adding dom0/domU support
if we conditionalize support in xen-sys but the API should
remain the same.
