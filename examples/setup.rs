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

use testx::testx;

#[allow(dead_code)]
fn setup() -> u32 {
    4711
}

#[allow(dead_code)]
fn setup_666() -> u32 {
    666
}

#[allow(dead_code)]
fn setup_2_args() -> (u32, String) {
    (4711, String::from("foo"))
}

#[testx]
fn sample_one_arg(num: u32) {
    assert_eq!(num, 4711);
}

#[testx(setup = setup_2_args)]
fn sample_two_args(num: u32, str: String) {
    assert_eq!(num, 4711);
    assert_eq!(str, "foo");
}

#[testx(setup = "setup_666")]
fn sample_custom_str(num: u32) {
    assert_eq!(num, 666);
}

#[testx(setup = setup_666)]
fn sample_custom_path(num: u32) {
    assert_eq!(num, 666);
}

#[testx(setup = self::setup_666)]
fn sample_custom_path2(num: u32) {
    assert_eq!(num, 666);
}

fn main() {}
