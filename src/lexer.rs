use std::{collections::HashMap, fmt};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Filter,
    Map,
    Chars,
    ToString,
    Keys,
    Values,
    Fold,
    Sort,
    Zip
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftCurl,
    RightCurl,
    Colon,
    LeftBracket,
    RightBracket,
    Comma,
    Num(f64),
    Bool(bool),
    Str(String),
    Var((isize, String)),
    Fn(Function),
    Macro(Vec<TokenData>),
    Nil,
    Plus,
    Minus,
    Mult,
    Div,
    Not,
    And,
    Or,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    LeftParen,
    RightParen,
    Dot,
    BackSlash,
    Sharp,
    Percent,
    Arrow,
    SemiColon,
    Question,
    Exclaim,
    Pipe,
    ConstraintMacro,
    ErrorMacro,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenData {
    pub token: Token,
    pub line: usize,
}

impl fmt::Display for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line:[{:04}] -> {:?}", self.line, self.token)?;

        Ok(())
    }
}

pub struct EconLexer<'a> {
    pub source: String,
    line: usize,
    start: usize,
    current: usize,
    macros: HashMap<String, (Vec<TokenData>, Vec<TokenData>)>,
    source_as_vec: Vec<&'a str>,
    current_string_read: String
}

impl<'a> EconLexer<'a> {
    pub fn init(source: &'a str) -> Self {
        Self {
            source: String::from(source),
            start: 0,
            current: 0,
            line: 0,
            macros: HashMap::new(),
            source_as_vec: source.graphemes(true).collect::<Vec<&'a str>>(),
            current_string_read: String::from("")
        }
    }
    
    fn error<T>(&self, msg: String) -> Result<T, String> {
        Err(format!("Line:[{:04}] -> Error Lexing -> {}", self.line, msg.clone()))
    }

    fn peek_prev(&self) -> Option<&str> {
        self.source_as_vec.get(self.current-1).copied()
    }
    
    fn peek(&self) -> Option<&str> {
        self.source_as_vec.get(self.current).copied()
    }
    
    fn peek_next(&self) -> Option<&str> {
        if self.at_end() { 
            None
        } else {
            self.source_as_vec.get(self.current+1).copied()
        }
    }
    
    fn advance(&mut self) -> Option<&str> {
        self.current += 1;
        self.source_as_vec.get(self.current-1).copied()
    }
    
    fn eat(&mut self) {
        current_string_read.push_str(self.peek().unwrap());
        self.current += 1;
    }
    
    fn make_token(&self, t: Token) -> Result<TokenData, String> {
        Ok(TokenData{ token: t, line: self.line })
    }
    
    fn skip_whitespace(&mut self) -> Result<(), String> {
        loop {
            match self.peek() {
                Some(" ") | Some("\t") => { self.eat(); }
                Some("/") => {
                    if let Some("/") = self.peek_next() {
                        loop {
                            self.eat();
                            if let Some("\n") = self.peek() {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                Some("\n") => { 
                    self.line += 1;
                    self.eat(); 
                }
                _ => { return Ok(()); }
            }
        }
        
        Ok(())
    }
     
    fn at_end(&self) -> bool {
        self.current >= self.source_as_vec.len()
    }
    
    fn is_digit(c: &str) -> bool {
        c >= "0" && c <= "9"
    }
    
    fn number(&mut self) -> Result<TokenData, String> {
        loop {
            if let Some(v) = self.peek() {
                if Self::is_digit(v) {
                    self.eat();
                } else {
                    break;
                }
            } else {
                return self.error("Unterminated Number.".to_string());
            }
        }
        
        if let (Some(v), Some(n)) = (self.peek(), self.peek_next()) {
            if v == "." && Self::is_digit(n) {
                self.eat();
                
                loop {
                    if let Some(v) = self.peek() {
                        if Self::is_digit(v) {
                            self.eat();
                        } else {
                            break;
                        }
                    } else {
                        return self.error("Unterminated Number.".to_string());
                    }
                }
            }
        }
        
        let build = self.current_string_read[self.start..self.current].to_string();
        let string_to_use = build.parse::<f64>();
            
        match string_to_use {
            Ok(val) => {
                self.make_token(Token::Num(val))
            }
            Err(_) => {
                self.error("Invalid Number.".to_string())
            }
        }
    }
    
    fn string(&mut self) -> Result<TokenData, String> {
        while let Some(v) = self.peek() {
            if v == "\\" {
                self.eat();
                self.eat();
            } else {
                if v != "\"" {
                    self.eat();
                } else {
                    break;
                }
            }
        }
        
        if self.at_end() {
            self.error("Unterminated String.".to_string())
        } else {
            let build = self.current_string_read[1..].to_string();
            self.eat();
            self.make_token(Token::Str(String::from(build)))
        }
    }
    
    fn variable(&mut self) -> Result<TokenData, String> {
        while let Some(v) = self.peek() {
            if let "/" | "*" | "+" | "-" | "(" | ")" | " " 
            | "\n" | "." | "," | "[" | "]" | ";" | ":" 
            | "|" | "@" | "%" = v {
                break;
            } else {
                self.eat();
            }
        }
        
        if self.at_end() {
            self.error("Unterminated Variable.".to_string())
        } else {
            let mut search = 0;
            while let Some(v) = self.source.graphemes(true).nth(self.start) {
                if let "!" = v {
                    search = -1;
                    break;
                }
            
                if let "$" = v {
                    search += 1;
                } else {
                    self.start -= 1;
                    search -= 1;
                    break;
                }
                self.start += 1;
            }
            
            let build = self.current_string_read[self.start+1..].to_string();
            Ok(TokenData{ token: Token::Var((search, build)), line: self.line})
        }
    }
    
    fn keyword(&mut self) -> Result<TokenData, String> {
        while let Some(v) = self.peek() {
            if !Self::is_alpha(v) { break; }
            self.eat();
        }

        let build = self.current_string_read[self.start..].to_string();
        
        if build == "true" {
            self.make_token(Token::Bool(true))
        } else if build == "false" {
            self.make_token(Token::Bool(false))
        } else if build == "nil" {
            self.make_token(Token::Nil)
        } else if build == "not" {
            self.make_token(Token::Not)
        } else if build == "or" {
            self.make_token(Token::Or)
        } else if build == "and" {
            self.make_token(Token::And)
        } else if build == "inf" {
            self.make_token(Token::Num(f64::INFINITY))
        } else if build == "filter" {
            self.make_token(Token::Fn(Function::Filter))
        } else if build == "map" {
            self.make_token(Token::Fn(Function::Map))
        } else if build == "chars" {
            self.make_token(Token::Fn(Function::Chars))
        } else if build == "to_string" {
            self.make_token(Token::Fn(Function::ToString))
        } else if build == "keys" {
            self.make_token(Token::Fn(Function::Keys))
        } else if build == "values" {
            self.make_token(Token::Fn(Function::Values))
        } else if build == "fold" {
            self.make_token(Token::Fn(Function::Fold))
        } else if build == "sort" {
            self.make_token(Token::Fn(Function::Sort))
        } else if build == "zip" {
            self.make_token(Token::Fn(Function::Zip))
        } else {
            while let Some(v) = self.peek() {
                if !Self::is_alpha(v) && !Self::is_digit(v) { break; }
                self.eat();
            }
            
            let build = self.current_string_read[self.start..].to_string();
            self.make_token(Token::Str(build))
        }
    }

    fn macro_t(&mut self) -> Result<TokenData, String> {
        self.eat();

        if let Token::Str(s) = (self.keyword()?).token {
            let macro_obj = if let Some(m) = self.macros.get(&s) {
                Some(m.clone())
            } else {
                None
            };
            
            if let Some(m) = macro_obj {
                if let Some("(") = self.peek() {
                    self.eat();
                    
                    let mut groupings = vec!();

                    if let Some(")") = self.peek() {
                        self.eat();
                    } else {
                        let mut current_group = vec!();
                        let mut depth = 1;
                        
                        loop {
                            let t = self.scan()?;
        
                            match &t {
                                TokenData{ token: Token::Comma, .. } => {
                                    let mut tmp = vec!();
                                    for i in current_group {
                                        tmp.push(i);
                                    }
                                    groupings.push(tmp);
                                    current_group = vec!();
                                    continue;
                                }
                                TokenData{ token: Token::Macro(tt), .. } => {
                                    for i in tt {
                                        current_group.push(i.clone());
                                    }
                                    continue;
                                }
                                TokenData{ token: Token::LeftParen, .. } => {
                                    depth += 1;
                                }
                                TokenData{ token: Token::RightParen, .. } => {
                                    depth -= 1;
                                    if depth <= 0 {
                                        let mut tmp = vec!();
                                        for i in current_group {
                                            tmp.push(i);
                                        }
                                        groupings.push(tmp);
                                        current_group = vec!();
                                        break;
                                    }
                                }
                                TokenData{ token: Token::EOF, .. } => {
                                    return self.error(format!("Unterminated Macro {}", s))
                                }
                                _ => { }
                            }
                            
                            current_group.push(t);
                        }
                    }
                    
                    if m.0.len() != groupings.len() {
                        self.error(format!("{} of {} args supplied to {}.", groupings.len(), m.0.len(), s))
                    } else {
                        let mut new_stream = vec!();
                        
                        'outer: for i in m.1 {
                            match i  {
                                TokenData{ token: Token::Str(ref s), .. } => {
                                    for (j, item) in m.0.iter().enumerate() {
                                        if let TokenData{ token: Token::Str(ss), .. } = item {
                                            if *s == *ss {
                                                for k in groupings[j].iter() {
                                                    new_stream.push(k.clone());
                                                }
                                                continue 'outer;
                                            }
                                        }
                                    }
                                    
                                    new_stream.push(i);
                                }
                                _ => {
                                    new_stream.push(i);
                                }
                            }
                        }
                        self.make_token(Token::Macro(new_stream))
                    }
                } else {
                    self.error(format!("Expect '(' after Macro {}.", s))
                }
            } else {
                if let Some("(") = self.peek() {
                    self.eat();
                    
                    let mut params = Vec::<TokenData>::new();
                
                    if let Some(")") = self.peek() {
                        self.eat();
                    } else {
                        loop {
                            match self.scan()? {
                                TokenData{ token: Token::Comma, .. } => {
                                    continue;
                                }
                                TokenData{ token: Token::RightParen, .. } => {
                                    break;
                                }
                                TokenData{ token: Token::EOF, .. } => {
                                    return self.error(format!("Unterminated Macro {}", s))
                                }
                                t => { params.push(t); }
                            }
                        }
                    }
                    
                    let mut stream = vec!();
                    loop {
                        while let Some(" ") | Some("\t") = self.peek() {
                            self.eat();
                        }
                        self.start = self.current;
                        
                        if let Some("\\") = self.peek() {
                            self.eat(); 
                            self.skip_whitespace()?;
                            self.start = self.current;
                        }
                        
                        if let Some("\n") = self.peek() {
                            break;
                        }
                        
                        stream.push(self.scan()?);
                    }
                    
                    self.macros.insert(s.clone(), (params, stream));

                    //a somewhat hacky way of telling the lexer to process the macro
                    Err("Macro".to_string()) 
                } else {
                    self.error(format!("Expect '(' after Macro {}.", s))
                }
            }
        } else {
            self.error("Unexpected Token.".to_string())
        }
    }
     
    fn is_alpha(c: &str) -> bool {
        (c >= "a" && c <= "z") ||
        (c >= "A" && c <= "Z") ||
        c == "_"
    }
    
    pub fn scan(&mut self) -> Result<TokenData, String> {
        self.skip_whitespace()?;
        self.current_string_read = String::from("");
        self.start = self.current;
        
        if self.at_end() { 
            self.make_token(Token::EOF)
        } else {
            match self.advance() {
                Some("{") => { self.make_token(Token::LeftCurl) }
                Some("}") => { self.make_token(Token::RightCurl) }
                Some("[") => { self.make_token(Token::LeftBracket) }
                Some("]") => { self.make_token(Token::RightBracket) }
                Some("(") => { self.make_token(Token::LeftParen) }
                Some(")") => { self.make_token(Token::RightParen) }
                Some(",") => { self.make_token(Token::Comma) }
                Some(":") => { self.make_token(Token::Colon) }
                Some("+") => { self.make_token(Token::Plus) }
                Some("-") => { self.make_token(Token::Minus) }
                Some(";") => { self.make_token(Token::SemiColon) }
                Some("?") => { self.make_token(Token::Question) }
                Some("*") => { self.make_token(Token::Mult) }
                Some("/") => { self.make_token(Token::Div) }
                Some(".") => { self.make_token(Token::Dot) }
                Some("\\") => { self.make_token(Token::BackSlash) }
                Some("#") => { self.make_token(Token::Sharp) }
                Some("%") => { self.make_token(Token::Percent) }
                Some("=") => {
                    match self.peek() {
                        Some("=") => {
                            self.eat();
                            self.make_token(Token::Equal)
                        }
                        Some(">") => {
                            self.eat();
                            self.make_token(Token::Arrow)
                        }
                        _ => { self.error("Unexpected Token.".to_string()) }
                    }
                }
                Some("&") => {
                    if let Some("&") = self.peek() {
                        self.eat();
                        self.make_token(Token::And)
                    } else {
                        self.error("Unexpected Token.".to_string())
                    }
                }
                Some("|") => {
                    if let Some("|") = self.peek() {
                        self.eat();
                        self.make_token(Token::Or)
                    } else {
                        self.make_token(Token::Pipe)
                    }
                }
                Some("~") => {
                    if let Some("=") = self.peek() {
                        self.eat();
                        self.make_token(Token::NotEqual)
                    } else {
                        self.make_token(Token::Not)
                    }
                }
                Some(">") => {
                    if let Some("=") = self.peek() {
                        self.eat();
                        self.make_token(Token::GreaterEqual)
                    } else {
                        self.make_token(Token::Greater)
                    }
                }
                Some("<") => {
                    if let Some("=") = self.peek() {
                        self.eat();
                        self.make_token(Token::LessEqual)
                    } else {
                        self.make_token(Token::Less)
                    }
                }
                Some("@") => { 
                    match self.peek() {
                        Some("{") => {
                            self.make_token(Token::ConstraintMacro)
                        }
                        Some("!") => {
                            self.eat();
                            self.make_token(Token::ErrorMacro)
                        }
                        _ => {
                            self.macro_t()
                        }
                    }
                }
                Some("\"") => { self.string() }
                Some("$") | Some("!") => { self.variable() }
                Some(v) => {
                    if Self::is_digit(v) {
                        self.number()
                    } else if Self::is_alpha(v)  {
                        self.keyword()
                    } else {
                        let v2 = self.peek();
                        self.error(format!("Unexpected Token got {:?}.", v2))
                    }
                }
                _ => { 
                    let v2 = self.peek();
                    self.error(format!("Unexpected Token got {:?}.", v2)) 
                }
            }
        }
    }
}
