use autocxx::prelude::*;
use cxx::let_cxx_string;

include_cpp! {
    // C++ headers we want to include.
    #include "myconfig.h"
    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe_ffi)
    // What types and functions we want to generate
    generate!("CppTest::MyConfig")
}

fn main() {
    let_cxx_string!(filename = "config.txt");
    let config = ffi::CppTest::MyConfig::from_file(&filename);

    println!("Config, name = {}", config.name());
}
