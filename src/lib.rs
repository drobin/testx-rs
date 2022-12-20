// MIT License
//
// Copyright (c) 2022 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! # An extended Rust testcase
//!
//! The `testx` crate provides the `testx` macro, which is an extended version
//! of the Rust `test` macro. The key features are:
//!
//! * The `testx` macro is fully compatible for the Rust `test` macro, all
//!   tests maked with `#[testx]` (instead of `#[test]`) are executed with
//!   `cargo-test`.
//! * Support for a test preparation function.
//!
//! ## Create a `testx` testcase
//!
//! Mark the testcase with `#[testx]`. Calling `cargo test` will execute the
//! testcase.
//!
//! ```rust
//! use testx::testx;
//!
//! #[testx]
//! fn sample() {
//!     assert_eq!(1, 1);
//! }
//!
//! // output:
//! // running 1 test
//! // test sample ... ok
//! ```
//!
//! ## Migrate from `#[test]` to `#[testx]`
//!
//! Simply replace the line `#[test]` with `#[testx]` for those tests you want
//! migrate.
//!
//! ## Prepare data for a testcase with a `setup` function
//!
//! Provide a function `setup` which prepares and returns some data for your
//! testcase. Next, your testcase needs one argument, which must match the
//! return value of the setup function.
//!
//! A testcase marked with `#[testx]` will first execute the `setup` function
//! and will pass its return value to your testcase.
//!
//! ```rust
//! use testx::testx;
//!
//! fn setup() -> u32 {
//!     4711
//! }
//!
//! #[testx]
//! pub fn sample(num: u32) {
//!     assert_eq!(num, 4711);
//! }
//! ```
//!
//! If more than one argument is required, put them all into a tuple:
//!
//! ```rust
//! use testx::testx;
//!
//! fn setup() -> (u32, String) {
//!     (4711, String::from("foo"))
//! }
//!
//! #[testx]
//! pub fn sample(data: (u32, String)) {
//!     let (num, str) = data;
//!
//!     assert_eq!(num, 4711);
//!     assert_eq!(str, "foo");
//! }
//! ```
//!
//! **Note**: For a testcase without an argument, the `setup` function will not
//! be executed!

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{ItemFn, Visibility};

fn has_arg(func: &ItemFn) -> bool {
    func.sig.inputs.iter().nth(0).is_some()
}

fn to_inner_func(func: &ItemFn) -> ItemFn {
    let ident_new = format!("{}_inner", func.sig.ident);
    let ident_new = syn::parse_str(&ident_new).unwrap();

    let mut inner = func.clone();
    inner.vis = Visibility::Inherited;
    inner.sig.ident = ident_new;
    inner.attrs.clear();

    inner
}

/// Macro marks an extended Rust testcase.
///
/// Refer to the [module documentation](self) for details about using the `testx`
/// macro.
#[proc_macro_attribute]
pub fn testx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let test_fn = parse_macro_input!(item as syn::ItemFn);
    let inner_fn = to_inner_func(&test_fn);

    let fn_arg = has_arg(&test_fn);
    let fn_attrs = &test_fn.attrs;
    let fn_ident = &test_fn.sig.ident;
    let inner_fn_ident = &inner_fn.sig.ident;

    let setup_call = if fn_arg {
        quote! {
            let sr = setup();
        }
    } else {
        quote! {}
    };

    let inner_call = if fn_arg {
        quote! {
            #inner_fn_ident(sr);
        }
    } else {
        quote! {
            #inner_fn_ident();
        }
    };

    quote! {
        #inner_fn

        #[test]
        #(#fn_attrs)*
        fn #fn_ident() {
            #setup_call
            #inner_call
        }
    }
    .into()
}
