// HACK: yaml読み取り

use std::fs;
extern crate yaml_rust;
use yaml_rust::YamlLoader;

#[allow(dead_code)]
pub fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}
