mod hypercall;
pub use hypercall::*;

mod bindgen {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("./bindgen.rs");
}
pub use bindgen::*;
