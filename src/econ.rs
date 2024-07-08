use std::{fs, path::PathBuf, result, str::FromStr};

use crate::{lexer::EconLexer, object::EconObj, parser::EconParser, value::EconValue};

pub struct Econ;

impl Econ {
    pub fn create(src: &str, debug: bool) -> Result<EconValue, String> {
        match PathBuf::from_str(src) {
            Ok(pb) => {
                let file = fs::read_to_string(pb);
                match file {
                    Ok(f) => {
                        let mut parser = EconParser::new(&f);
                        let mut lexer = EconLexer::init(&f);
                        let result = parser.parse(&mut lexer, debug);
                        if debug {
                            if let Err(e) = &result {
                                eprintln!("{}", e);
                            }
                        }
                        result
                    }
                    Err(e) => {
                        let mut parser = EconParser::new(src);
                        let mut lexer = EconLexer::init(src);
                        let result = parser.parse(&mut lexer, debug);
                        if debug {
                            if let Err(e) = &result {
                                eprintln!("{}", e);
                            }
                        }
                        result
                    }
                }
            }
            Err(_) => {
                let mut parser = EconParser::new(src);
                let mut lexer = EconLexer::init(src);
                let result = parser.parse(&mut lexer, debug);
                if debug {
                    if let Err(e) = &result {
                        eprintln!("{}", e);
                    }
                }
                result
            }
        }
    }

    pub fn from(src: &str) -> EconValue {
        match PathBuf::try_from(src) {
            Ok(pb) => {
                match fs::read_to_string(pb) {
                    Ok(s) => {
                        match Self::create(&s, false) {
                            Ok(v) => {
                                v
                            }
                            Err(m) => {
                                eprintln!("{}", m);
                                vec!(EconValue::Obj(EconObj::new()))
                            }
                        }
                    }
                    Err(_) => {
                        match Self::create(src, false) {
                            Ok(v) => {
                                v
                            }
                            Err(m) => {
                                eprintln!("{}", m);
                                vec!(EconValue::Obj(EconObj::new()))
                            }
                        }
                    }
                }
            }
            Err(_) => {
                match Self::create(src, false) {
                    Ok(v) => {
                        v
                    }
                    Err(m) => {
                        eprintln!("{}", m);
                        vec!(EconValue::Obj(EconObj::new()))
                    }
                }
            }
        }
    }

    pub fn to_struct<T: for<'a> serde::de::Deserialize<'a>>(obj: &EconValue) -> Result<T, String> {
        if let EconValue::Obj(o) = &obj {
            let result: Result<T, serde_json::Error> = serde_json::from_str(o.stringify().as_str());
            result.map_err(|e| e.to_string())
        } else {
            let result: Result<T, serde_json::Error> = serde_json::from_str(format!("{}", obj).as_str());
            result.map_err(|e| e.to_string())
        }
    }
}
