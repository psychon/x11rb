#![deny(//missing_copy_implementations,
        //missing_debug_implementations,
        //missing_docs,
        //private_doc_tests,
        //single_use_lifetimes,
        trivial_casts,
        trivial_numeric_casts,
        //unreachable_pub
        )]

pub mod wire;
pub mod thoughts;
pub mod xcb_ffi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
