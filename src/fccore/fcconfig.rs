use rustc_serialize::json;
use std::fs::File;
use std::io::{Write, Read};

#[derive(RustcEncodable, RustcDecodable)]
pub struct FCConfig {
  fc_serve_port : usize,
  status_pin : usize,
  arm_switch_pin : usize
}

impl FCConfig {
  fn read_config_file(base_file : &str) -> String {
    let mut result = String::new();
    File::open(base_file).unwrap().read_to_string(&mut result);
    return result;
  }
  pub fn new(base_file : &str) -> FCConfig {
    //let text = FCConfig::read_config_file(base_file);
    //return json::decode(&text).unwrap();
    FCConfig{fc_serve_port:0, status_pin:0, arm_switch_pin:0}
  }
}