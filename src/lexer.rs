use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Filter,
    Map,
    Chars,
    String,
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
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenData {
    pub token: Token,
    pub line: usize,
    pub section: (usize, usize)
}

impl TokenData {
    pub fn get_section(&self, src: &String) -> String {
        src[self.section.0..self.section.1].to_string()
    }
}

impl fmt::Display for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line:[{:04}] -> {:?}", self.line, self.token)?;

        Ok(())
    }
}

pub struct EconLexer {
    pub source: String,
    line: usize,
    section: (usize, usize),
    start: usize,
    current: usize,
    macros: HashMap<String, (Vec<TokenData>, Vec<TokenData>)>
}

impl EconLexer {
    pub fn init(source: &str) -> Self {
        Self {
            source: String::from(source),
            start: 0,
            current: 0,
            line: 0,
            section: (0, 0),
            macros: HashMap::new()
        }
    }
    
    fn error<T>(&self, msg: String) -> Result<T, String> {
        Err(format!("Line:[{:04}] -> Error Lexing -> {}", self.line, msg.clone()))
    }
    
    fn get_section(&self) -> (usize, usize) {
        self.section
    }
    
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    
    fn peek_next(&self) -> Option<char> {
        if self.at_end() { 
            None
        } else {
            self.source.chars().nth(self.current+1)
        }
    }
    
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current-1)
    }
    
    fn eat(&mut self) {
        self.current += 1;
    }
    
    fn make_token(&self, t: Token) -> Result<TokenData, String> {
        Ok(TokenData{ token: t, line: self.line, section: self.get_section()})
    }
    
    fn skip_whitespace(&mut self) -> Result<(), String> {
        loop {
            match self.peek() {
                Some(' ') | Some('\t') => { self.eat(); }
                Some('/') => {
                    if let Some('/') = self.peek_next() {
                        loop {
                            self.eat();
                            if let Some('\n') = self.peek() {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
                Some('\n') => { 
                    self.line += 1;
                    let mut chars = self.source.chars();
                    let chars_len = self.source.chars().count();

                    let mut count = 0;
                    
                    for i in (0..self.current).rev() {
                        if let Some('\n') = chars.nth(i) {
                            count += 1;
                        }
                        
                        if count > 1 {
                            self.section.0 = i;
                            break;
                        }
                    }
                    
                    count = 0;
                    
                    for i in self.current+1..chars_len {
                        if let Some('\n') = chars.nth(i) {
                            count += 1;
                        }
                        
                        if count > 0 {
                            self.section.1 = i;
                            break;
                        }
                    }
                    
                    self.eat(); 
                }
                _ => { return Ok(()); }
            }
        }
        
        Ok(())
    }
     
    fn at_end(&self) -> bool {
        self.source.chars().nth(self.current).is_none()
    }
    
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
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
            if v == '.' && Self::is_digit(n) {
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
        
        let string_to_use = String::from(&self.source[self.start..self.current]).parse::<f64>();
            
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
            if v != '"' {
                self.eat();
            } else {
                break;
            }
        }
        
        if self.at_end() {
            self.error("Unterminated String.".to_string())
        } else {
            self.eat();
            self.make_token(Token::Str(String::from(&self.source[self.start+1..self.current-1])))
        }
    }
    
    fn variable(&mut self) -> Result<TokenData, String> {
        while let Some(v) = self.peek() {
            if let '/' | '*' | '+' | '-' | '(' | ')' | ' ' | '\n' | '.' | ',' | '[' | ']' | ';' | ':' | '|' | '@' = v {
                break;
            } else {
                self.eat();
            }
        }
        
        if self.at_end() {
            self.error("Unterminated Variable.".to_string())
        } else {
            let mut search = 0;
            while let Some(v) = self.source.chars().nth(self.start) {
                if let '!' = v {
                    search = -1;
                    break;
                }
            
                if let '$' = v {
                    search += 1;
                } else {
                    self.start -= 1;
                    search -= 1;
                    break;
                }
                self.start += 1;
            }
        
            Ok(TokenData{ token: Token::Var((search, String::from(&self.source[self.start+1..self.current]))), line: self.line, section: self.get_section()})
        }
    }
    
    fn keyword(&mut self) -> Result<TokenData, String> {
        while let Some(v) = self.peek() {
            if !Self::is_alpha(v) { break; }
            self.eat();
        }
        
        if &self.source[self.start..self.current] == "true" {
            self.make_token(Token::Bool(true))
        } else if &self.source[self.start..self.current] == "false" {
            self.make_token(Token::Bool(false))
        } else if &self.source[self.start..self.current] == "nil" {
            self.make_token(Token::Nil)
        } else if &self.source[self.start..self.current] == "not" {
            self.make_token(Token::Not)
        } else if &self.source[self.start..self.current] == "or" {
            self.make_token(Token::Or)
        } else if &self.source[self.start..self.current] == "and" {
            self.make_token(Token::And)
        } else if &self.source[self.start..self.current] == "filter" {
            self.make_token(Token::Fn(Function::Filter))
        } else if &self.source[self.start..self.current] == "map" {
            self.make_token(Token::Fn(Function::Map))
        } else if &self.source[self.start..self.current] == "chars" {
            self.make_token(Token::Fn(Function::Chars))
        } else if &self.source[self.start..self.current] == "string" {
            self.make_token(Token::Fn(Function::String))
        } else if &self.source[self.start..self.current] == "keys" {
            self.make_token(Token::Fn(Function::Keys))
        } else if &self.source[self.start..self.current] == "values" {
            self.make_token(Token::Fn(Function::Values))
        } else if &self.source[self.start..self.current] == "fold" {
            self.make_token(Token::Fn(Function::Fold))
        } else if &self.source[self.start..self.current] == "sort" {
            self.make_token(Token::Fn(Function::Sort))
        } else if &self.source[self.start..self.current] == "zip" {
            self.make_token(Token::Fn(Function::Zip))
        } else {
            while let Some(v) = self.peek() {
                if !Self::is_alpha(v) && !Self::is_digit(v) { break; }
                self.eat();
            }
            
            self.make_token(Token::Str(String::from(&self.source[self.start..self.current])))
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
                if let Some('(') = self.peek() {
                    self.eat();
                    
                    let mut groupings = vec!();

                    if let Some(')') = self.peek() {
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
                    self.error("Expect '(' token after Macro identifier.".to_string())
                }
            } else {
                if let Some('(') = self.peek() {
                    self.eat();
                    
                    let mut params = Vec::<TokenData>::new();
                
                    if let Some(')') = self.peek() {
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
                                t => { params.push(t); }
                            }
                        }
                    }
                    
                    let mut stream = vec!();
                    loop {
                        while let Some(' ') | Some('\t') = self.peek() {
                            self.eat();
                        }
                        self.start = self.current;
                        
                        if let Some('\\') = self.peek() {
                            self.eat(); 
                            self.skip_whitespace()?;
                            self.start = self.current;
                        }
                        
                        if let Some('\n') = self.peek() {
                            break;
                        }
                        
                        stream.push(self.scan()?);
                    }
                    
                    self.macros.insert(s.clone(), (params, stream));
                    Err("Macro".to_string())
                } else {
                    self.error("Expect '(' token after Macro identifier.".to_string())
                }
            }
        } else {
            self.error("Unexpected Token.".to_string())
        }
    }
     
    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }
    
    pub fn scan(&mut self) -> Result<TokenData, String> {
        self.skip_whitespace()?;
        self.start = self.current;
        
        if self.at_end() { 
            self.make_token(Token::EOF)
        } else {
            match self.advance() {
                Some('{') => { self.make_token(Token::LeftCurl) }
                Some('}') => { self.make_token(Token::RightCurl) }
                Some('[') => { self.make_token(Token::LeftBracket) }
                Some(']') => { self.make_token(Token::RightBracket) }
                Some('(') => { self.make_token(Token::LeftParen) }
                Some(')') => { self.make_token(Token::RightParen) }
                Some(',') => { self.make_token(Token::Comma) }
                Some(':') => { self.make_token(Token::Colon) }
                Some('+') => { self.make_token(Token::Plus) }
                Some('-') => { self.make_token(Token::Minus) }
                Some(';') => { self.make_token(Token::SemiColon) }
                Some('?') => { self.make_token(Token::Question) }
                Some('*') => { self.make_token(Token::Mult) }
                Some('/') => { self.make_token(Token::Div) }
                Some('.') => { self.make_token(Token::Dot) }
                Some('\\') => { self.make_token(Token::BackSlash) }
                Some('#') => { self.make_token(Token::Sharp) }
                Some('%') => { self.make_token(Token::Percent) }
                Some('=') => {
                    match self.peek() {
                        Some('=') => {
                            self.eat();
                            self.make_token(Token::Equal)
                        }
                        Some('>') => {
                            self.eat();
                            self.make_token(Token::Arrow)
                        }
                        _ => { self.error("Unexpected Token.".to_string()) }
                    }
                }
                Some('&') => {
                    if let Some('&') = self.peek() {
                        self.eat();
                        self.make_token(Token::And)
                    } else {
                        self.error("Unexpected Token.".to_string())
                    }
                }
                Some('|') => {
                    if let Some('|') = self.peek() {
                        self.eat();
                        self.make_token(Token::Or)
                    } else {
                        self.make_token(Token::Pipe)
                    }
                }
                Some('~') => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        self.make_token(Token::NotEqual)
                    } else {
                        self.make_token(Token::Not)
                    }
                }
                Some('>') => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        self.make_token(Token::GreaterEqual)
                    } else {
                        self.make_token(Token::Greater)
                    }
                }
                Some('<') => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        self.make_token(Token::LessEqual)
                    } else {
                        self.make_token(Token::Less)
                    }
                }
                Some('@') => { self.macro_t() }
                Some('"') => { self.string() }
                Some('$') | Some('!') => { self.variable() }
                Some(v) => {
                    if Self::is_digit(v) {
                        self.number()
                    } else if Self::is_alpha(v)  {
                        self.keyword()
                    } else {
                        self.error("Unexpected Token.".to_string())
                    }
                }
                _ => { self.error("Unexpected Token.".to_string()) }
            }
        }
    }
}