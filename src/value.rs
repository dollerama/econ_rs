use std::fmt;

use crate::object::JsonObj;

#[derive(Debug, Clone)]
pub enum JsonValue {
    Nil,
    Num(f64),
    Bool(bool),
    Str(String),
    Arr(Vec<JsonValue>),
    Obj(JsonObj)
}

impl From<bool> for JsonValue {
    fn from(item: bool) -> Self {
        JsonValue::Bool(item)
    }
}
impl From<&JsonValue> for bool {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Bool(v) = item {
            *v as bool
        } else {
            false
        }
    }
}

impl From<String> for JsonValue {
    fn from(item: String) -> Self {
        JsonValue::Str(item)
    }
}
impl From<&JsonValue> for String {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Str(v) = item {
            v.clone()
        } else {
            "Undefined".to_string()
        }
    }
}

impl From<&str> for JsonValue {
    fn from(item: &str) -> Self {
        JsonValue::Str(String::from(item))
    }
}

impl From<i8> for JsonValue {
    fn from(item: i8) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for i8 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as i8
        } else {
            0i8
        }
    }
}

impl From<i16> for JsonValue {
    fn from(item: i16) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for i16 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as i16
        } else {
            0i16
        }
    }
}

impl From<i32> for JsonValue {
    fn from(item: i32) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for i32 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as i32
        } else {
            0i32
        }
    }
}

impl From<i64> for JsonValue {
    fn from(item: i64) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for i64 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as i64
        } else {
            0i64
        }
    }
}

impl From<isize> for JsonValue {
    fn from(item: isize) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for isize {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as isize
        } else {
            0isize
        }
    }
}

impl From<u8> for JsonValue {
    fn from(item: u8) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for u8 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as u8
        } else {
            0u8
        }
    }
}

impl From<u16> for JsonValue {
    fn from(item: u16) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for u16 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as u16
        } else {
            0u16
        }
    }
}

impl From<u32> for JsonValue {
    fn from(item: u32) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for u32 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as u32
        } else {
            0u32
        }
    }
}

impl From<u64> for JsonValue {
    fn from(item: u64) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for u64 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as u64
        } else {
            0u64
        }
    }
}

impl From<usize> for JsonValue {
    fn from(item: usize) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for usize {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as usize
        } else {
            0usize
        }
    }
}

impl From<f32> for JsonValue {
    fn from(item: f32) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for f32 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as f32
        } else {
            0f32
        }
    }
}

impl From<f64> for JsonValue {
    fn from(item: f64) -> Self {
        JsonValue::Num(item as f64)
    }
}
impl From<&JsonValue> for f64 {
    fn from(item: &JsonValue) -> Self {
        if let JsonValue::Num(v) = item {
            *v as f64
        } else {
            0f64
        }
    }
}

impl JsonValue {
    const NIL: JsonValue = JsonValue::Nil;

    pub fn key(&self, key: &str) -> &JsonValue {
        match self {
            JsonValue::Obj(o) => {
                if let Some(val) = o.key(key) {
                    val
                } else {
                    &Self::NIL
                }
            }
            _ => { &Self::NIL }
        }
    }
    
    pub fn mut_key(&mut self, key: &str) -> Option<&mut JsonValue> {
        match self {
            JsonValue::Obj(o) => {
                o.key_mut(key)
            }
            _ => { None }
        }
    }
    
    pub fn index(&self, ind: usize) -> &JsonValue {
        match self {
            JsonValue::Arr(o) => {
                if let Some(val) = o.get(ind) {
                    val
                } else {
                    &Self::NIL
                }
            }
            _ => { &Self::NIL }
        }
    }
    
    pub fn mut_index(&mut self, ind: usize) -> Option<&mut JsonValue> {
        match self {
            JsonValue::Arr(o) => {
                o.get_mut(ind)
            }
            _ => { None }
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Obj(o) => {
                write!(f, "{}", o.get_string_from_obj(o, 0))
            }
            JsonValue::Arr(a) => {
                let b = JsonObj::new();
                write!(f, "[\n{}", b.get_string_from_arr(a, 0))
            }
            _ => write!(f, "{:?}", self)
        }
    }
}