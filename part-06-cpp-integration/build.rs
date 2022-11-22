// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("cpp/src");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path]).build()?;
    b.flag_if_supported("-std=c++17")
        .file("cpp/src/myconfig.cc")
        .compile("autocxx-part-06");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/src/myconfig.h");
    println!("cargo:rerun-if-changed=cpp/src/myconfig.cpp");
    Ok(())
}
