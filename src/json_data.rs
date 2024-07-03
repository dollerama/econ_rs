use std::{fmt, fs, path::PathBuf};

use crate::{lexer::JsonLexer, parser::JsonParser, value::JsonValue};

pub struct Json {
    pub data: Vec<JsonValue>,
}

impl Json {
    const NIL: JsonValue = JsonValue::Nil;
    
    pub fn index(&self, i: usize) -> &JsonValue {
        if let Some(val) = self.data.get(i) {
            val
        } else {
            &Self::NIL
        }
    }
    
    pub fn mut_index(&mut self, i: usize) -> Option<&mut JsonValue> {
        self.data.get_mut(i)
    }

    pub fn create(src: &str, debug: bool) -> Result<Self, String> {
        let mut parser = JsonParser::new(src);
        let mut lexer = JsonLexer::init(src);
        parser.parse(&mut lexer, debug)
    }
}

impl From<&str> for Json {
    fn from(src: &str) -> Self {
        match Self::create(src, false) {
            Ok(v) => {
                v
            }
            Err(m) => {
                eprintln!("{}", m);
                Json { data: Vec::<JsonValue>::new() }
            }
        }
    }
}

impl From<PathBuf> for Json {
    fn from(p: PathBuf) -> Self {
        let file = fs::read_to_string(p);
        match file {
            Ok(src) => {
                match Self::create(&src, false) {
                    Ok(v) => {
                        v
                    }
                    Err(m) => {
                        eprintln!("{}", m);
                        Json { data: Vec::<JsonValue>::new() }
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                Json {
                    data: vec!()
                }
            }
        }
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.data.len() {
    
            if let JsonValue::Obj(o) = &self.data[i] {
                write!(f, "{}", o.get_string_from_obj(o, 0))?;
            } else {
                write!(f, "{:?}", self.data[i])?;
            }
            
            if i+1 < self.data.len() {
                write!(f, ",\n")?;
            }
        }
        
        Ok(())
    }
}