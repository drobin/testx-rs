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

use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, Path, Token};

struct SetupFunc(Path);

impl Parse for SetupFunc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            Ok(SetupFunc(lit.parse()?))
        } else if lookahead.peek(Ident::peek_any) {
            Ok(SetupFunc(input.call(Path::parse_mod_style)?))
        } else {
            Err(lookahead.error())
        }
    }
}

enum Attribute {
    Setup(Option<SetupFunc>),
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        let setup = if key == "setup" {
            let _: Token![=] = input.parse()?;
            Some(input.parse()?)
        } else if key == "no_setup" {
            None
        } else {
            return Err(syn::Error::new_spanned(
                key,
                "unsupported attribute for testx",
            ));
        };

        Ok(Attribute::Setup(setup))
    }
}

pub struct AttributeList {
    attrs: Punctuated<Attribute, Token![,]>,
    def_setup: Path,
}

impl AttributeList {
    fn default_setup_func() -> syn::Result<Path> {
        syn::parse_str("setup")
    }

    pub fn setup_func(&self) -> Option<&Path> {
        for attr in self.attrs.iter() {
            match attr {
                Attribute::Setup(opt) => {
                    return opt.as_ref().map(|func| &func.0);
                }
            };
        }

        Some(&self.def_setup)
    }
}

impl Parse for AttributeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = if input.is_empty() {
            Punctuated::new()
        } else {
            input.call(Punctuated::parse_separated_nonempty)?
        };

        let def_setup = Self::default_setup_func()?;

        Ok(AttributeList { attrs, def_setup })
    }
}
