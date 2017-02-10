extern crate rjson;

use rjson::rjson_type::*;

fn main() {
    println!("Hello, world!");
    let rjson_value = rjson_value { rj_type: rjson_type::RJ_NULL };
    println!("{:?}", rjson_value)
}
