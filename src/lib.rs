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
//!
//! By default a function called `setup` is called for each testcase. If you
//! need another function, assign a `setup` attribute with the name of the
//! setup function to the macro. The function name can be either a string or
//! the path to the function. Assign the `no_setup` attribute if you want to
//! explicity mark a testcase to have no setup function.
//!
//! ```rust
//! use testx::testx;
//!
//! fn setup_666() -> u32 {
//!     666
//! }
//!
//! #[testx(no_setup)]
//! fn sample_no_setup() {
//!     assert_eq!(1, 1);
//! }
//!
//! #[testx(setup = "setup_666")]
//! pub fn sample_custom_str(num: u32) {
//!     assert_eq!(num, 666);
//! }
//!
//! #[testx(setup = setup_666)]
//! pub fn sample_custom_path(num: u32) {
//!     assert_eq!(num, 666);
//! }
//!
//! #[testx(setup = self::setup_666)]
//! pub fn sample_custom_path2(num: u32) {
//!     assert_eq!(num, 666);
//! }
//! ```
//!

mod attr;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::{ItemFn, Visibility};

use crate::attr::AttributeList;

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
pub fn testx(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input.
    let meta = parse_macro_input!(attr as AttributeList);
    let test_fn = parse_macro_input!(item as syn::ItemFn);

    // The inner function is based on the test-function and contains the test logic.
    let inner_fn = to_inner_func(&test_fn);

    // The configured setup function.
    let setup_fn = meta.setup_func();

    // Attributes of the test function.
    let fn_arg = has_arg(&test_fn);
    let fn_attrs = &test_fn.attrs;
    let fn_ident = &test_fn.sig.ident;
    let inner_fn_ident = &inner_fn.sig.ident;

    // you need to call setup, if
    // * you have an configured setup function
    // * the test-function has an argument (where the setup result is passed to)
    let need_setup = setup_fn.is_some() && fn_arg;

    let setup_call = if need_setup {
        quote! {
            let sr = #setup_fn();
        }
    } else {
        quote! {}
    };

    let inner_call = if need_setup {
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
