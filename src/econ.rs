use std::{fs, path::PathBuf, result, str::FromStr};

use crate::{lexer::EconLexer, object::EconObj, parser::EconParser, value::EconValue};

/// Parse Econ from strings or files. Access values directly or deserialize into rust structs.
/// # Examples
/// ```rust
/// use econ_rs::econ::Econ;
///
/// let obj = Econ::from(
/// r#"
/// {
///     a: {
///         b: {
///             c: [
///                 1,
///                 2,
///                 3,
///                 4
///             ]
///         }
///     }
/// }
/// "#);
/// assert_eq!(3f64, obj["a"]["b"]["c"][2].value::<f64>());
/// ```
/// ```rust
/// use econ_rs::econ::Econ;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct Point {
///     x: f64,
///     y: f64
/// }
///
/// let mut p = Point {x: 0.0, y: 0.0};
/// let obj = Econ::from(
/// r#"
/// {
///     x: 1+1,
///     y: 2+5
/// }
/// "#);
/// ```
pub struct Econ;

impl Econ {
    /// create an EconValue
    /// # Example
    /// ```rust
    /// use econ_rs::econ::Econ;
    ///
    /// let obj = Econ::create(
    /// r#"
    /// {
    ///     a: 1,
    ///     b: 2,
    ///     c: 3
    /// }
    /// "#, true);
    /// ```
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

    /// create an EconValue from ```&str``` or file path. Does not include any debug info. Prints error message and returns ```EconValue::Nil``` on fail.
    /// # Examples
    /// ```rust
    /// use econ_rs::econ::Econ;
    ///
    /// let obj = Econ::from(
    /// r#"
    /// {
    ///     a: 1,
    ///     b: 2,
    ///     c: 3
    /// }
    /// "#);
    /// ```
    /// ```rust,ignore
    /// use econ_rs::econ::Econ;
    ///
    /// let obj = Econ::from("path/file.econ");
    /// ```
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
                                EconValue::Nil
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
                                EconValue::Nil
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
                        EconValue::Nil
                    }
                }
            }
        }
    }

    /// Deserialize Econ into a struct. Econ is a superset of Json so it utilizes serde to deserialize to struct.
    /// # Examples
    /// ```rust
    /// use econ_rs::econ::Econ;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Point {
    ///     x: f64,
    ///     y: f64
    /// }
    ///
    /// let mut p = Point {x: 0.0, y: 0.0};
    /// let obj = Econ::from(
    /// r#"
    /// {
    ///     x: 1+1,
    ///     y: 2+5
    /// }
    /// "#);
    /// ```
    pub fn to_struct<T: for<'a> serde::de::Deserialize<'a>>(obj: &EconValue) -> Result<T, String> {
        let result: Result<T, serde_json::Error> = serde_json::from_str(format!("{}", obj).as_str());
        result.map_err(|e| e.to_string())
    }
}
