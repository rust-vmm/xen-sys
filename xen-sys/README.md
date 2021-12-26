# Goal

This project aims to provide unsafe calls to the Xen Hypervisor
hypercall API. It is aimed for kernel level (or unikernel)
style projects and not is not currently intended to be used
from user space Linux applications. Potentially in the future
we can make its calls via /dev/xen/ interfaces while bare metal
and kernel systems will natively call the hypercalls. This crate
could export different modules for the /dev/xen interfaces and
allow the xen crate to provide the same API to the user by
using `#[cfg()]` blocks.
