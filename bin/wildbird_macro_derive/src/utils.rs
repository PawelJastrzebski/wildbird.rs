use proc_macro::TokenStream;
use std::collections::HashMap;
use proc_macro2::TokenStream as TokenStream2;
use syn::{ItemStruct, Visibility};

pub fn is_public(service_struct: &ItemStruct) -> bool {
    match service_struct.vis {
        Visibility::Public(_) => { true }
        _ => false
    }
}

pub fn parse_attr_to_map(attr: TokenStream) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let raw = attr.to_string();

    for entry in raw.split(",") {
        let key_value = entry.split("=").collect::<Vec<&str>>();
        let key = key_value.get(0).unwrap_or(&"");
        let value = key_value.get(1).unwrap_or(&"");
        map.insert(key.trim().to_string(), value.trim().replace("\"", "").to_string());
    }
    map
}