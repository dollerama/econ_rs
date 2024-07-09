use std::fmt;

use indexmap::IndexMap;

use crate::value::EconValue;

pub trait Access<T> {
    fn get(&self, i: T) -> &EconValue;
    fn get_mut(&mut self, i: T) -> Option<&mut EconValue>;
}

#[derive(Debug, Clone)]
pub struct EconObj {
    pub data: IndexMap<String, EconValue>
}

impl fmt::Display for EconObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_string_from_obj(self, 0))
    }
}

impl EconObj {
    const NIL: EconValue = EconValue::Nil;

    pub fn new() -> Self {
        Self {
            data: IndexMap::new()
        }
    }

    pub fn stringify(&self) -> String {
        format!("{}", self)
    }

    pub fn get_string_from_arr(&self, arr: &Vec<EconValue>, depth: usize) -> String {
        let mut result = String::new();
        let mut i = 0;
    
        for v in arr.iter() {
            for _ in 0..depth+1 {
                result.push_str("\t");
            }
            if let EconValue::Arr(a) = v {
                result.push_str(&format!("[\n"));

                if i+1 < arr.len() {
                    result.push_str(&format!("{},\n", &self.get_string_from_arr(&a, depth+1)));
                } else {
                    result.push_str(&format!("{}\n", &self.get_string_from_arr(&a, depth+1)));
                }            
            } else {
                match v {
                    EconValue::Bool(b) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("{},\n", b));
                        } else {
                            result.push_str(&format!("{}\n", b));
                        }
                    }
                    EconValue::Num(n) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("{},\n", n));
                        } else {
                            result.push_str(&format!("{}\n", n));
                        }
                    }
                    EconValue::Str(s) => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("\"{}\",\n", s));
                        } else {
                            result.push_str(&format!("\"{}\"\n", s));
                        }
                    }
                    EconValue::Nil => {
                        if i+1 < arr.len() {
                            result.push_str(&format!("nil,\n"));
                        } else {
                            result.push_str(&format!("nil\n"));
                        }
                    }
                    EconValue::Obj(o) => {
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
        
    pub fn get_string_from_obj(&self, obj: &EconObj, depth: usize) -> String {
        let mut result = String::new();
        let mut i = 0;
        
        result.push_str(&format!("{{\n"));

        for (k, v) in obj.data.iter() {
            for _ in 0..depth+1 {
                result.push_str("\t");
            }
            if let EconValue::Obj(o) = v {
                result.push_str(&format!("\"{}\": ", k));
                
                result.push_str(&self.get_string_from_obj(&o, depth+1));
                
                if i+1 < obj.data.keys().len() {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            } else {
                match v {
                    EconValue::Bool(b) => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("\"{}\": {},\n", k, b));
                        } else {
                            result.push_str(&format!("\"{}\": {}\n", k, b));
                        }
                    }
                    EconValue::Num(n) => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("\"{}\": {},\n", k, n));
                        } else {
                            result.push_str(&format!("\"{}\": {}\n", k, n));
                        }
                    }
                    EconValue::Str(s) => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("\"{}\": \"{}\",\n", k, s));
                        } else {
                            result.push_str(&format!("\"{}\": \"{}\"\n", k, s));
                        }
                    }
                    EconValue::Nil => {
                        if i+1 < obj.data.keys().len() {
                            result.push_str(&format!("\"{}\": nil,\n", k));
                        } else {
                            result.push_str(&format!("\"{}\": nil\n", k));
                        }
                    }
                    EconValue::Arr(a) => {
                        result.push_str(&format!("\"{}\": [\n", k));
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

impl Access<&str> for EconObj {
    fn get(&self, key: &str) -> &EconValue {
        match self.data.get(key) {
            Some(v) => v,
            None => &Self::NIL
        }
    }
    
    fn get_mut(&mut self, key: &str) -> Option<&mut EconValue> {
        self.data.get_mut(key)
    }
}