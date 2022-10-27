# Contributing to xen-bindings

## Dependencies

### Bindgen
The bindings are currently generated using
[bindgen](https://crates.io/crates/bindgen) version 0.61.0:
```bash
cargo install bindgen --vers 0.61.0
```

### Xen Hypervisor 
Generating bindings depends on the Xen source code, so you need to have the
repository on your machine:

```bash
git clone https://github.com/xen-project/xen.git
```

## Updating to a new Xen version

Two environment variable, ARCH and XEN_DIR,  need to be updated in the build.sh
script to regenerate the Xen bindgins:

```bash
# x86_64 or aarch64                         
ARCH="x86_64"
    
# Path to Xen project source code
XEN_DIR="/path/to/xen/project/xen/"
```

Failure to do so will result in a failure to regenerate the bindings.  Once the
environment variables have been setup simply run the build.sh script.  The
resulting files will be either bindings_x86_64.rs or bindings_aarch64.rs,
depending on the targeted architecture.

```bash

$ ./build.sh
$ copy bindings_x86_64.rs src/
```
