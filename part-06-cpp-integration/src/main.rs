use autocxx::prelude::*;
use cxx::let_cxx_string;

include_cpp! {
    // C++ headers we want to include.
    #include "myconfig.h"
    // Safety policy
    safety!(unsafe_ffi)
    // What types and functions we want to generate
    generate!("CppTest::MyConfig")
}

fn main() {
    let_cxx_string!(filename = "config.txt");
    let config = ffi::CppTest::MyConfig::from_file(&filename);

    println!(
        "Config, name = {}, width = {}, height = {}",
        config.name(),
        config.width(),
        config.height()
    );
}
