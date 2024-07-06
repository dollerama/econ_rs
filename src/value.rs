use std::fmt;

use crate::object::{Access, EconObj};

#[derive(Debug, Clone)]
pub enum EconValue {
    Nil,
    Num(f64),
    Bool(bool),
    Str(String),
    Arr(Vec<EconValue>),
    Obj(EconObj)
}

impl From<bool> for EconValue {
    fn from(item: bool) -> Self {
        EconValue::Bool(item)
    }
}
impl From<&EconValue> for bool {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Bool(v) = item {
            *v as bool
        } else {
            false
        }
    }
}

impl From<String> for EconValue {
    fn from(item: String) -> Self {
        EconValue::Str(item)
    }
}
impl From<&EconValue> for String {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Str(v) = item {
            v.clone()
        } else {
            "Undefined".to_string()
        }
    }
}

impl From<&str> for EconValue {
    fn from(item: &str) -> Self {
        EconValue::Str(String::from(item))
    }
}

impl From<i8> for EconValue {
    fn from(item: i8) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for i8 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as i8
        } else {
            0i8
        }
    }
}

impl From<i16> for EconValue {
    fn from(item: i16) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for i16 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as i16
        } else {
            0i16
        }
    }
}

impl From<i32> for EconValue {
    fn from(item: i32) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for i32 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as i32
        } else {
            0i32
        }
    }
}

impl From<i64> for EconValue {
    fn from(item: i64) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for i64 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as i64
        } else {
            0i64
        }
    }
}

impl From<isize> for EconValue {
    fn from(item: isize) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for isize {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as isize
        } else {
            0isize
        }
    }
}

impl From<u8> for EconValue {
    fn from(item: u8) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for u8 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as u8
        } else {
            0u8
        }
    }
}

impl From<u16> for EconValue {
    fn from(item: u16) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for u16 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as u16
        } else {
            0u16
        }
    }
}

impl From<u32> for EconValue {
    fn from(item: u32) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for u32 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as u32
        } else {
            0u32
        }
    }
}

impl From<u64> for EconValue {
    fn from(item: u64) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for u64 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as u64
        } else {
            0u64
        }
    }
}

impl From<usize> for EconValue {
    fn from(item: usize) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for usize {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as usize
        } else {
            0usize
        }
    }
}

impl From<f32> for EconValue {
    fn from(item: f32) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for f32 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as f32
        } else {
            0f32
        }
    }
}

impl From<f64> for EconValue {
    fn from(item: f64) -> Self {
        EconValue::Num(item as f64)
    }
}
impl From<&EconValue> for f64 {
    fn from(item: &EconValue) -> Self {
        if let EconValue::Num(v) = item {
            *v as f64
        } else {
            0f64
        }
    }
}

impl EconValue {
    const NIL: EconValue = EconValue::Nil;
    
    pub fn value<T: for<'a> std::convert::From<&'a EconValue>>(&self) -> T {
        let res : T = self.into();
        res
    } 
}

impl Access<&str> for EconValue {
    
    fn get(&self, i: &str) -> &EconValue {
        match self {
            EconValue::Obj(o) => {
                o.get(i)
            }
            _ => { &Self::NIL }
        }
    }
    
    fn get_mut(&mut self, i: &str) -> Option<&mut EconValue> {
        match self {
            EconValue::Obj(o) => {
                o.get_mut(i)
            }
            _ => { None }
        }
    }
}

impl Access<usize> for EconValue {
    fn get(&self, i: usize) -> &EconValue {
        match self {
            EconValue::Arr(o) => {
                if let Some(val) = o.get(i) {
                    val
                } else {
                    &Self::NIL
                }
            }
            _ => { &Self::NIL }
        }
    }
    
    fn get_mut(&mut self, i: usize) -> Option<&mut EconValue> {
        match self {
            EconValue::Arr(o) => {
                o.get_mut(i)
            }
            _ => { None }
        }
    }
}

impl fmt::Display for EconValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EconValue::Obj(o) => {
                write!(f, "{}", o.get_string_from_obj(o, 0))
            }
            EconValue::Arr(a) => {
                let b = EconObj::new();
                write!(f, "[\n{}", b.get_string_from_arr(a, 0))
            }
            _ => write!(f, "{:?}", self)
        }
    }
}