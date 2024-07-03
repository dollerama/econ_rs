use std::{collections::HashMap, fmt};

use crate::value::JsonValue;

#[derive(Debug, Clone)]
pub struct JsonObj {
    pub data: HashMap<String, JsonValue>
}

impl fmt::Display for JsonObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_string_from_obj(self, 0))
    }
}

impl JsonObj {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }
    
    pub fn key(&self, key: &str) -> Option<&JsonValue> {
        self.data.get(key)
    }
    
    pub fn key_mut(&mut self, key: &str) -> Option<&mut JsonValue> {
        self.data.get_mut(key)
    }
    
    pub fn get_string_from_arr(&self, arr: &Vec<JsonValue>, depth: usize) -> String {
        let mut result = String::new();
        let mut i = 0;
    
        for v in arr.iter() {
            for _ in 0..depth+1 {
                result.push_str("\t");
            }
            if let JsonValue::Arr(a) = v {
                result.push_str(&format!("[\n"));

                if i+1 < arr.len() {
                    result.push_str(&format!("{},\n", &self.get_string_from_arr(&a, depth+1)));
                } else {
                    result.push_str(&format!("{}\n", &self.get_string_from_arr(&a, depth+1)));
                }            
            } else {
                match v {
                    JsonValue::Bool(b) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("{},\n", b));
                        } else {
                            result.push_str(&format!("{}\n", b));
                        }
                    }
                    JsonValue::Num(n) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("{},\n", n));
                        } else {
                            result.push_str(&format!("{}\n", n));
                        }
                    }
                    JsonValue::Str(s) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("{},\n", s));
                        } else {
                            result.push_str(&format!("{}\n", s));
                        }
                    }
                    JsonValue::Nil => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("nil,\n"));
                        } else {
                            result.push_str(&format!("nil\n"));
                        }
                    }
                    JsonValue::Obj(o) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("{},\n", &self.get_string_from_obj(&o, depth+1)));
                        } else {
                            result.push_str(&format!("{}\n", &self.get_string_from_obj(&o, depth+1)));
                        }
                    }
                    _ => {} 
                }
            }
            i += 1;
        }
        
        for _ in 0..depth {
            result.push_str("\t");
        }
        result.push_str(&format!("]"));
        
        result
    }
        
    pub fn get_string_from_obj(&self, obj: &JsonObj, depth: usize) -> String {
        let mut result = String::new();
        let mut i = 0;
        
        result.push_str(&format!("{{\n"));

        for (k, v) in obj.data.iter() {
            for _ in 0..depth+1 {
                result.push_str("\t");
            }
            if let JsonValue::Obj(o) = v {
                result.push_str(&format!("{}: ", k));
                
                result.push_str(&self.get_string_from_obj(&o, depth+1));
                
                if i+1 < obj.data.keys().len() {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            } else {
                match v {
                    JsonValue::Bool(b) => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("{}: {},\n", k, b));
                        } else {
                            result.push_str(&format!("{}: {}\n", k, b));
                        }
                    }
                    JsonValue::Num(n) => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("{}: {},\n", k, n));
                        } else {
                            result.push_str(&format!("{}: {}\n", k, n));
                        }
                    }
                    JsonValue::Str(s) => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("{}: {},\n", k, s));
                        } else {
                            result.push_str(&format!("{}: {}\n", k, s));
                        }
                    }
                    JsonValue::Nil => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("{}: nil,\n", k));
                        } else {
                            result.push_str(&format!("{}: nil\n", k));
                        }
                    }
                    JsonValue::Arr(a) => {
                        result.push_str(&format!("{}: [\n", k));
                        result.push_str(&self.get_string_from_arr(&a, depth+1));
                        if i+1 < obj.data.keys().len() {
                            result.push_str(",\n");
                        } else {
                            result.push('\n');
                        }
                    }
                    _ => {} 
                }
            }
            
            i += 1;
        }
        for _ in 0..depth {
            result.push_str("\t");
        }
        result.push_str(&format!("}}"));
        
        result
    }
}