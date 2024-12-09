mod hypercall;
pub use hypercall::*;

mod bindgen {
    #![allow(
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        deref_nullptr,
        clippy::redundant_static_lifetimes
    )]
    include!("./bindgen.rs");
}
pub use bindgen::*;
