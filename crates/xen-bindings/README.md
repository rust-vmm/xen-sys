# xen-bindings
Rust FFI bindings for the Xen hypervisor and xentools generated using [bindgen](https://crates.io/crates/bindgen).

# Usage
Add this to your `Cargo.toml`:
```toml
xen-bindings = { git = "https://github.com/rust-vmm/xen-sys/", version = "v0.1.0" }
```
You can then import the bindings where you need them:
```rust
use xen_bindings::bindings::

or

use xen_bindings::bindings::{xs_watch_type, xs_watch_type_XS_WATCH_PATH};
```
