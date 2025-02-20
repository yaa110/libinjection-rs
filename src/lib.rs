//! # libinjection
//!
//! Rust bindings for (libinjection)][1]
//!
//! [1]: https://github.com/libinjection/libinjection
//!
//! ## How to Use
//!
//! ```
//! use libinjection::{sqli, xss};
//!
//! let (is_sqli, fingerprint) = sqli("' OR '1'='1' --").unwrap();
//! assert!(is_sqli);
//! assert_eq!("s&sos", fingerprint);
//!
//! let is_xss = xss("<script type='text/javascript'>alert('xss');</script>").unwrap();
//! assert!(is_xss);
//! ```
//!

#![doc(html_root_url = "https://docs.rs/libinjection")]
#![deny(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;
mod wrapper;

pub use wrapper::{sqli, xss};
