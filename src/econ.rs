use std::{fs, path::PathBuf, str::FromStr};

use crate::{lexer::EconLexer, object::EconObj, parser::EconParser};

pub type Econ = EconObj;

impl Econ {
    pub fn create(src: &str, debug: bool) -> Result<EconObj, String> {
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
}

impl From<&str> for Econ {
    fn from(src: &str) -> Econ {
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
                                EconObj::new()
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
                                EconObj::new()
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
                        EconObj::new()
                    }
                }
            }
        }
    }
}

impl From<PathBuf> for Econ {
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
                        EconObj::new()
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                EconObj::new()
            }
        }
    }
}