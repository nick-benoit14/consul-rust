#![allow(non_snake_case)]

use std::str::from_utf8;

use rustc_serialize::json;
use curl::http;

pub struct Keystore{
    endpoint: String
}

impl Keystore {
    pub fn new(address: &str) ->  Keystore {
        Keystore {
            endpoint: format!("http://{}/v1/kv", address)
        }
    }
    
    pub fn set_key(&self, key: String, value: String) {
        let url = format!("{}/{}", self.endpoint, key);
        let resp = http::handle()
            .put(url, &value)
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Consul: Error setting a key!");
        }
    }

    pub fn acquire_lock(&self, key: String, address: &String, session_id: &String) -> bool {
        let url;
        if key.to_owned().into_bytes()[0] == 0x2f {
            url = format!("{}{}?acquire={}", self.endpoint, key, session_id);
        }
        else {
            url = format!("{}/{}?acquire={}", self.endpoint, key, session_id);
        }
        let resp = http::handle()
            .put(url, address)
            .content_type("application/json")
            .exec().unwrap();
        if resp.get_code() != 200 {
            panic!("Consul: Error acquiring a lock!");
        }
        let result = from_utf8(resp.get_body()).unwrap();
        if result == "true" {
            return true;
        }
        false
    }
    
    pub fn get_key(&self, key: String) -> String {
        let url;
        if key.to_owned().into_bytes()[0] == 0x2f {
            url = format!("{}{}", self.endpoint, key);
        }
        else {
            url = format!("{}/{}", self.endpoint, key);
        }
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        if resp.get_code() != 200 {
            return String::new();
        }
        let json_data = match json::Json::from_str(result) {
            Ok(value) => value,
            Err(_) => panic!("consul: Could not convert to json: {:?}", result)
        };
        let v_json = json_data.as_array().unwrap();
        super::get_string(&v_json[0], &["Value"])
    }

    pub fn delete_key(&self, key: String) {
        let url;
        if key.to_owned().into_bytes()[0] == 0x2f {
            url = format!("{}{}", self.endpoint, key);
        }
        else {
            url = format!("{}/{}", self.endpoint, key);
        }
        let resp = http::handle().delete(url).exec().unwrap();
        let _result = from_utf8(resp.get_body()).unwrap();
        if resp.get_code() != 200 {
            panic!("Could not delete key: {}", key);
        }
    }
    
}