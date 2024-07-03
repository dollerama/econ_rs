use std::{collections::HashMap, time::Instant};

use crate::{json_data::Json, lexer::{Function, JsonLexer, Token, TokenData}, object::JsonObj, value::JsonValue};

pub struct JsonParser { 
    tokens: Vec<TokenData>,
    current: usize,
    source: String,
    locals: Vec<HashMap<String, JsonValue>>,
    depth: isize
}

impl JsonParser {
    pub fn new(src: &str) -> Self {
        Self {
            tokens: vec!(),
            current: 0,
            source: String::from(src),
            locals: vec!(),
            depth: -1
        }
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current].token
    }
    
    fn peek_full(&self) -> &TokenData {
        &self.tokens[self.current]
    }
    
    fn advance(&mut self) -> &Token {
        self.current += 1;
        match self.tokens.get(self.current) {
            Some(v) => { 
                &v.token
            }
            None => { &self.tokens[self.current-1].token }
        }
    }
    
    fn eat(&mut self) {
        self.current += 1;
    }
    
    fn at_end(&self) -> bool {
        match self.tokens.get(self.current) {
            Some(_) => { false }
            None => { true }
        }
    }
    
    fn check(&self, t: Token) -> bool {
        !self.at_end() && *self.peek() == t 
    }
    
    fn match_single(&mut self, t1: Token) -> bool {
        if self.check(t1) {
            self.eat();
            true
        } else {
            false
        }
    }
    
    fn error<T>(&self, msg: String) -> Result<T, String> {
        let mut result_err = String::from("");
        
        result_err.push_str(&format!("Line [{:04}] Error Parsing -> \"{}\"\n", self.peek_full().line, msg.clone()));
        
        let data = self.peek_full().clone();
        let current_line = data.line;
        let total_lines = data.get_section(&self.source).lines().count();
        
        for (num, line) in data.get_section(&self.source).lines().enumerate() {
            if num == 0 { continue; }
            let actual_line_num = (current_line as i32 - total_lines as i32) + (num as i32 + 1i32);
            if actual_line_num < 0 {
                continue;
            }
            
            if actual_line_num == current_line as i32 {
                result_err.push_str(&format!("-> [{:04}]{}\n", actual_line_num, line));
            } else {
                result_err.push_str(&format!("[{:04}]   {}\n", actual_line_num, line));
            }
        }
        
        Err(result_err)
    }
    
    fn consume(&mut self, t: Token, msg: String) -> Result<&Token, String>  {
        if self.check(t) { 
            Ok(self.advance())
        } else {
            self.error(msg.clone())
        }
    }
    
    fn equality(&mut self) -> Result<JsonValue, String> {
        let mut left = self.comparison()?;
        
        loop {
            match self.peek().clone() {
                Token::Equal => {
                    self.eat();
                    let right = self.comparison()?;

                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Bool(n1==n2)
                        }
                        (&JsonValue::Bool(ref n1), &JsonValue::Bool(ref n2)) => {
                            JsonValue::Bool(n1==n2)
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Str(ref n2)) => {
                            JsonValue::Bool(n1==n2)
                        }
                        _ => return self.error("Invalid comparison of types".to_string())
                    };
                }
                Token::NotEqual => {
                    self.eat();
                    let right = self.comparison()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Bool(n1!=n2)
                        }
                        (&JsonValue::Bool(ref n1), &JsonValue::Bool(ref n2)) => {
                            JsonValue::Bool(n1!=n2)
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Str(ref n2)) => {
                            JsonValue::Bool(n1!=n2)
                        }
                        _ => return self.error("Invalid comparison of types.".to_string())
                    };
                }
                Token::Question => {
                    self.eat();
                    let right1 = self.equality()?;
                    self.consume(Token::Colon, "Expect ':'.".to_string())?;
                    let right2 = self.equality()?;
                    
                    left = match &left {
                        &JsonValue::Bool(true) => {
                            right1
                        }
                        &JsonValue::Bool(false) => {
                            right2
                        }
                        _ => return self.error("Expected Bool from ?.".to_string())
                    };
                }
                _ => { break; }
            }
        }
        
        Ok(left)
    }
    
    fn comparison(&mut self) -> Result<JsonValue, String> {
        let mut left = self.term()?;
        
        loop {
            match self.peek() {
                Token::Less => {
                    self.eat();
                    let right = self.term()?;
                    
                    left = match (&left, &right) {
                        (JsonValue::Num(n1), JsonValue::Num(n2)) => {
                            JsonValue::Bool(*n1 < *n2)
                        }
                        (JsonValue::Str(n1), JsonValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = c2 as u32 - 48 > c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            JsonValue::Bool(res)
                        }
                        _ => return self.error("Invalid comparison of types.".to_string())
                    };
                }
                Token::Greater => {
                    self.eat();
                    let right = self.term()?;
                    left = match (&left, &right) {
                        (JsonValue::Num(n1), JsonValue::Num(n2)) => {
                            JsonValue::Bool(*n1 > *n2)
                        }
                        (JsonValue::Str(n1), JsonValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = (c2 as u32 - 48) < c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            JsonValue::Bool(res)
                        }
                        _ => return self.error("Invalid comparison of types.".to_string())
                    };
                }
                Token::GreaterEqual => {
                    self.eat();
                    let right = self.term()?;
                    left = match (&left, &right) {
                        (JsonValue::Num(n1), JsonValue::Num(n2)) => {
                            JsonValue::Bool(*n1 >= *n2)
                        }
                        (JsonValue::Str(n1), JsonValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = (c2 as u32 - 48) < c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            JsonValue::Bool(res)
                        }
                        _ => return self.error("Invalid comparison of types.".to_string())
                    };
                }
                Token::LessEqual => {
                    self.eat();
                    let right = self.term()?;
                    left = match (&left, &right) {
                        (JsonValue::Num(n1), JsonValue::Num(n2)) => {
                            JsonValue::Bool(*n1 <= *n2)
                        }
                        (JsonValue::Str(n1), JsonValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = c2 as u32 - 48 > c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            JsonValue::Bool(res)
                        }
                        _ => return self.error("Invalid comparison of types.".to_string())
                    };
                }
                Token::And => {
                    self.eat();
                    let right = self.term()?;
                    if let (JsonValue::Bool(n1), JsonValue::Bool(n2)) = (&left, right) {
                        left = JsonValue::Bool(*n1 && n2);
                    } 
                }
                Token::Or => {
                    self.eat();
                    let right = self.term()?;
                    if let (JsonValue::Bool(n1), JsonValue::Bool(n2)) = (&left, right) {
                        left = JsonValue::Bool(*n1 || n2);
                    } 
                }
                _ => { break; }
            }
        }
        
        Ok(left)
    }
    
    fn term(&mut self) -> Result<JsonValue, String> {
        let mut left = self.factor()?;
        
        loop {
            match self.peek() {
                Token::Plus => {
                    self.eat();
                    let right = self.factor()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Num(n1+n2)
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Str(ref n2)) =>  {
                            JsonValue::Str(format!("{}{}", n1, n2))
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Str(format!("{}{}", n1, n2))
                        }
                        (&JsonValue::Num(ref n1), &JsonValue::Str(ref n2)) =>  {
                            JsonValue::Str(format!("{}{}", n1, n2))
                        }
                        (&JsonValue::Num(ref n1), &JsonValue::Nil) =>  {
                            JsonValue::Num(*n1)
                        }
                        (&JsonValue::Nil, &JsonValue::Num(ref n1)) =>  {
                            JsonValue::Num(*n1)
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Nil) =>  {
                            JsonValue::Str(format!("{}", n1))
                        }
                        (&JsonValue::Nil, &JsonValue::Str(ref n1)) =>  {
                            JsonValue::Str(format!("{}", n1))
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Bool(ref n2)) =>  {
                            JsonValue::Str(format!("{}{}", n1, n2))
                        }
                        (&JsonValue::Bool(ref n1), &JsonValue::Str(ref n2)) =>  {
                            JsonValue::Str(format!("{}{}", n1, n2))
                        }
                        (&JsonValue::Arr(ref n1), &JsonValue::Arr(ref n2)) =>  {
                            let mut new_arr = vec!();
                            
                            for a in n1 {
                                new_arr.push(a.clone());
                            }
                            for a in n2 {
                                new_arr.push(a.clone());
                            }

                            JsonValue::Arr(new_arr)
                        }
                        (&JsonValue::Arr(ref n1), &JsonValue::Nil) =>  {
                            let mut new_arr = vec!();
                            
                            for a in n1 {
                                new_arr.push(a.clone());
                            }

                            JsonValue::Arr(new_arr)
                        }
                        (&JsonValue::Nil, &JsonValue::Arr(ref n1)) =>  {
                            let mut new_arr = vec!();
                            
                            for a in n1 {
                                new_arr.push(a.clone());
                            }

                            JsonValue::Arr(new_arr)
                        }
                        (&JsonValue::Obj(ref n1), &JsonValue::Obj(ref n2)) =>  {
                            let mut new_obj = JsonObj::new();
                            
                            for (k, v) in &n1.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }
                            for (k, v) in &n2.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }

                            JsonValue::Obj(new_obj)
                        }
                        (&JsonValue::Obj(ref n1), &JsonValue::Nil) =>  {
                            let mut new_obj = JsonObj::new();
                            
                            for (k, v) in &n1.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }
                            
                            JsonValue::Obj(new_obj)
                        }
                        (&JsonValue::Nil, &JsonValue::Obj(ref n1)) =>  {
                            let mut new_obj = JsonObj::new();
                            
                            for (k, v) in &n1.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }
                            
                            JsonValue::Obj(new_obj)
                        }
                        _ => return self.error("Invalid addition of types.".to_string())
                    };
                }
                Token::BackSlash => {
                    self.eat();
                    let right = self.factor()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Str(ref n1), &JsonValue::Str(ref n2)) =>  {
                            JsonValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&JsonValue::Num(ref n1), &JsonValue::Str(ref n2)) =>  {
                            JsonValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Nil) =>  {
                            JsonValue::Str(format!("{}", n1))
                        }
                        (&JsonValue::Nil, &JsonValue::Str(ref n1)) =>  {
                            JsonValue::Str(format!("{}", n1))
                        }
                        (&JsonValue::Str(ref n1), &JsonValue::Bool(ref n2)) =>  {
                            JsonValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&JsonValue::Bool(ref n1), &JsonValue::Str(ref n2)) =>  {
                            JsonValue::Str(format!("{}\n{}", n1, n2))
                        }
                        _ => return self.error("Invalid concatenation of types.".to_string())
                    };
                }
                Token::Minus => {
                    self.eat();
                    let right = self.factor()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Num(n1-n2)
                        }
                        _ => return self.error("Invalid subtraction of types.".to_string())
                    };
                }
                _ => {
                    break;
                }
            }
        }
        
        Ok(left)
    }
    
    fn factor(&mut self) -> Result<JsonValue, String> {
        let mut left = self.unary()?;
        
        loop {
            match self.peek() {
                Token::Mult => {
                    self.eat();
                    let right = self.unary()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Num(n1*n2)
                        }
                        _ => return self.error("Invalid multiplication of types.".to_string())
                    };
                }
                Token::Div => {
                    self.eat();
                    let right = self.unary()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Num(n1/n2)
                        }
                        _ => return self.error("Invalid division of types.".to_string())
                    };
                }
                Token::Percent => {
                    self.eat();
                    let right = self.unary()?;
                    
                    left = match (&left, &right) {
                        (&JsonValue::Num(ref n1), &JsonValue::Num(ref n2)) =>  {
                            JsonValue::Num(n1.rem_euclid(*n2))
                        }
                        _ => return self.error("Invalid modulus of types.".to_string())
                    };
                }
                _ => {
                    break;
                }
            }
        }
        
        Ok(left)
    }
    
    fn unary(&mut self) -> Result<JsonValue, String> {
        match self.peek().clone() {
            Token::Minus => {
                self.eat();
                let right = self.unary()?;
                if let JsonValue::Num(n1) = right {
                    Ok(JsonValue::Num(-n1))
                } else {
                    Ok(right)
                }
            }
            Token::Not => {
                self.eat();
                let right = self.unary()?;
                if let JsonValue::Bool(b1) = right {
                    Ok(JsonValue::Bool(!b1))
                } else {
                    Ok(right)
                }
            }
            Token::Sharp => {
                self.eat();
                let right = self.unary()?;
                match &right {
                    &JsonValue::Str(ref n1) => {
                        Ok(JsonValue::Num(n1.chars().count() as f64))
                    }
                    &JsonValue::Num(ref n1) => {
                        Ok(JsonValue::Num(*n1))
                    }
                    &JsonValue::Arr(ref n1) => {
                        Ok(JsonValue::Num(n1.len() as f64))
                    }
                    &JsonValue::Obj(ref n1) => {
                        Ok(JsonValue::Num(n1.data.keys().len() as f64))
                    }
                    _ => return self.error("Invalid measurement of type.".to_string())
                }
            }
            _ => {
                self.primary()
            }
        }
    }
    
    fn filter_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            JsonValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_vec = vec!();
                for (j, aa) in a.iter().enumerate() {
                    if let JsonValue::Str(ref s) = &temp_1.0 {
                        self.locals[self.depth as usize].insert(s.clone(), aa.clone());
                    } 
                    let goto_point = self.current;
                    let condition = self.val_expression()?;
                    
                    match condition {
                        JsonValue::Bool(true) => {
                            new_vec.push(aa.clone());
                        }
                        JsonValue::Bool(false) => { }
                        _ => {
                            return self.error("Filter condition must be boolean.".to_string());
                        }
                    }
                    if j < a.len()-1 {
                        self.current = goto_point;
                    }
                }
    
                self.restore_temp_var(temp_1);
                
                self.consume(Token::RightParen, "Expect ')' after Function.".to_string())?; 
                Ok(JsonValue::Arr(new_vec))
            }
            JsonValue::Obj(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_obj = JsonObj::new();
                for (j, aa) in a.data.iter().enumerate() {
                    if let JsonValue::Str(ref s) = &temp_1.0 {
                        let mut key_val = JsonObj::new();
                        key_val.data.insert("key".to_string(), JsonValue::Str(aa.0.clone()));
                        key_val.data.insert("val".to_string(), aa.1.clone());
                        self.locals[self.depth as usize].insert(s.clone(), 
                            JsonValue::Obj(key_val)
                        );
                    } 
                    let goto_point = self.current;
                    let condition = self.val_expression()?;
                    
                    match condition {
                        JsonValue::Bool(true) => {
                            new_obj.data.insert(aa.0.clone(), aa.1.clone());
                        }
                        JsonValue::Bool(false) => { }
                        _ => {
                            return self.error("Filter condition must be boolean.".to_string());
                        }
                    }
                    if j < a.data.keys().len()-1 {
                        self.current = goto_point;
                    }
                }
                
                self.restore_temp_var(temp_1);
                
                self.consume(Token::RightParen, "Expect ')' after Map Function.".to_string())?; 
                Ok(JsonValue::Obj(new_obj))
            }
            _ => {
                return self.error("Invalid literal/variable in function arguments.".to_string());
            }
        }
    }
    
    fn keys_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Keys Function.".to_string())?;
        let right = self.val_expression()?;
        
        if let JsonValue::Obj(s) = right {
            self.consume(Token::RightParen, "Expect ')' after Keys Function.".to_string())?; 
            let mut new_vec = vec!();
            for i in s.data.keys() {
                new_vec.push(JsonValue::Str(i.to_string()));
            }
            Ok(JsonValue::Arr(new_vec))
        } else {
            return self.error("Invalid literal/variable in Keys Function arguments.".to_string());
        }
    }
    
    fn values_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Keys Function.".to_string())?;
        let right = self.val_expression()?;
        
        if let JsonValue::Obj(s) = right {
            self.consume(Token::RightParen, "Expect ')' after Keys Function.".to_string())?; 
            let mut new_vec = vec!();
            for i in s.data.values() {
                new_vec.push(i.clone());
            }
            Ok(JsonValue::Arr(new_vec))
        } else {
            return self.error("Invalid literal/variable in Keys Function arguments.".to_string());
        }
    }
    
    fn chars_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Chars Function.".to_string())?;
        let right = self.val_expression()?;
        
        if let JsonValue::Str(s) = right {
            self.consume(Token::RightParen, "Expect ')' after Chars Function.".to_string())?; 
            let mut new_vec = vec!();
            for i in s.chars() {
                new_vec.push(JsonValue::Str(i.to_string()));
            }
            Ok(JsonValue::Arr(new_vec))
        } else {
            return self.error("Invalid literal/variable in Chars Function arguments.".to_string());
        }
    }
    
    fn string_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after String Function.".to_string())?;
        let right = self.val_expression()?;

        if let &JsonValue::Arr(ref a) = &right {
            self.consume(Token::RightParen, "Expect ')' after String Function.".to_string())?; 
            let mut new_str = String::from("");
            for i in a {
                if let JsonValue::Str(s) = i {
                    new_str.push_str(s);
                }
            }
            Ok(JsonValue::Str(new_str))
        } else {
            return self.error("Invalid literal/variable in String Function arguments.".to_string());
        }
    }
    
    fn map_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Map Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            JsonValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_vec = vec!();
                for (j, aa) in a.iter().enumerate() {
                    if let JsonValue::Str(ref s) = &temp_1.0 {
                        self.locals[self.depth as usize].insert(s.clone(), aa.clone());
                    } 
                    let goto_point = self.current;
                    let expr = self.val_expression()?;
                    
                    new_vec.push(expr);
    
                    if j < a.len()-1 {
                        self.current = goto_point;
                    }
                }
                
                self.restore_temp_var(temp_1);
                
                self.consume(Token::RightParen, "Expect ')' after Map Function.".to_string())?; 
                Ok(JsonValue::Arr(new_vec))
            }
            JsonValue::Obj(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_obj = JsonObj::new();
                for (j, aa) in a.data.iter().enumerate() {
                    if let JsonValue::Str(ref s) = &temp_1.0 {
                        let mut key_val = JsonObj::new();
                        key_val.data.insert("key".to_string(), JsonValue::Str(aa.0.clone()));
                        key_val.data.insert("val".to_string(), aa.1.clone());
                        self.locals[self.depth as usize].insert(s.clone(), 
                            JsonValue::Obj(key_val)
                        );
                    } 
                    let goto_point = self.current;
                    let expr = self.val_expression()?;
                    
                    new_obj.data.insert(aa.0.clone(), expr);
    
                    if j < a.data.keys().len()-1 {
                        self.current = goto_point;
                    }
                }
                
                self.restore_temp_var(temp_1);
                
                self.consume(Token::RightParen, "Expect ')' after Map Function.".to_string())?; 
                Ok(JsonValue::Obj(new_obj))
            }
            _ => {
                self.error("Invalid literal/variable in Map Function arguments.".to_string())
            }
        }
    }
    
    fn create_temp_var(&mut self) -> Result<(JsonValue, Option<JsonValue>), String> {
        let i_name = self.val_expression()?;
        let cached_val;
                
        if let JsonValue::Str(ref s) = &i_name {
            cached_val = if let Some(v) = self.locals[self.depth as usize].get(s) {
                Some(v.clone())
            } else {
                None
            };
            self.locals[self.depth as usize].insert(s.clone(), JsonValue::Nil);
        } else {
            return self.error("Invalid iterator identifier.".to_string())
        }
        
        Ok((i_name.clone(), cached_val.clone()))
    }
    
    fn restore_temp_var(&mut self, var: (JsonValue, Option<JsonValue>)) {
        if let (JsonValue::Str(ref s), Some(cache)) = (&var.0, var.1) {
            self.locals[self.depth as usize].insert(s.clone(), cache.clone());
        }
    }
    
    fn fold_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Fold Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            JsonValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                self.consume(Token::Pipe, "Expect '|' before args.".to_string())?;
                let temp_1 = self.create_temp_var()?;
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                let temp_2 = self.create_temp_var()?;
                self.consume(Token::Pipe, "Expect '|' after args.".to_string())?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;

                for (j, aa) in a.iter().enumerate() {
                    if let JsonValue::Str(ref s) = &temp_1.0 {
                        self.locals[self.depth as usize].insert(s.clone(), 
                            aa.clone()
                        );
                    }
                    
                    if let JsonValue::Str(ref s) = &temp_2.0 {
                        if let None = self.locals[self.depth as usize].get(s) {
                            self.locals[self.depth as usize].insert(s.clone(), JsonValue::Nil);
                        }
                    }
                    
                    let goto_point = self.current;
                    let expr = self.val_expression()?;
                    
                    if let JsonValue::Str(ref s) = &temp_2.0 {
                        self.locals[self.depth as usize].insert(s.clone(), expr);
                    }

                    if j < a.len()-1 {
                        self.current = goto_point;
                    }
                }
                
                let ret_val = if let JsonValue::Str(ref s) = &temp_2.0 {
                    if let Some(rv) = self.locals[self.depth as usize].get(s) {
                        rv.clone()
                    } else {
                        JsonValue::Nil
                    }
                } else {
                    JsonValue::Nil
                };
                
                self.restore_temp_var(temp_1);
                self.restore_temp_var(temp_2);
                
                self.consume(Token::RightParen, "Expect ')' after Fold Function.".to_string())?; 
                Ok(ret_val)
            }
            JsonValue::Obj(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                self.consume(Token::Pipe, "Expect '|' before args.".to_string())?;
                let temp_1 = self.create_temp_var()?;
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                let temp_2 = self.create_temp_var()?;
                self.consume(Token::Pipe, "Expect '|' after args.".to_string())?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;

                for (j, aa) in a.data.iter().enumerate() {
                    if let JsonValue::Str(ref s) = &temp_1.0 {
                        let mut key_val = JsonObj::new();

                        key_val.data.insert("key".to_string(), JsonValue::Str(aa.0.clone()));
                        key_val.data.insert("val".to_string(), aa.1.clone());
                        self.locals[self.depth as usize].insert(s.clone(), 
                            JsonValue::Obj(key_val)
                        );
                    }
                    
                    if let JsonValue::Str(ref s) = &temp_2.0 {
                        if let None = self.locals[self.depth as usize].get(s) {
                            self.locals[self.depth as usize].insert(s.clone(), JsonValue::Nil);
                        }
                    }
                    
                    let goto_point = self.current;
                    let expr = self.val_expression()?;
                    
                    if let JsonValue::Str(ref s) = &temp_2.0 {
                        self.locals[self.depth as usize].insert(s.clone(), expr);
                    }

                    if j < a.data.keys().len()-1 {
                        self.current = goto_point;
                    }
                }
                
                let ret_val = if let JsonValue::Str(ref s) = &temp_2.0 {
                    if let Some(rv) = self.locals[self.depth as usize].get(s) {
                        rv.clone()
                    } else {
                        JsonValue::Nil
                    }
                } else {
                    JsonValue::Nil
                };
                
                self.restore_temp_var(temp_1);
                self.restore_temp_var(temp_2);
                
                self.consume(Token::RightParen, "Expect ')' after Fold Function.".to_string())?; 
                Ok(ret_val)
            }
            _ => {
                self.error("Invalid literal/variable in Fold Function arguments.".to_string())
            }
        }
    }

    fn zip_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Zip Function.".to_string())?;
        let a = self.val_expression()?;
        self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
        let b = self.val_expression()?;
        
        let res = match (a, b) {
            (JsonValue::Arr(aa), JsonValue::Arr(bb)) => {
                let mut i = 0;
                let mut ret = vec!();
                loop {
                    match (aa.get(i), bb.get(i)) {
                        (Some(av), Some(bv)) => {
                            ret.push(JsonValue::Arr(vec![av.clone(), bv.clone()]))
                        }
                        (Some(av), None) => {
                            ret.push(JsonValue::Arr(vec![av.clone(), JsonValue::Nil]))
                        }
                        (None, Some(bv)) => {
                            ret.push(JsonValue::Arr(vec![JsonValue::Nil, bv.clone()]))
                        }
                        (None, None) => { break; }
                    }
                    i += 1;
                }
                
                ret
            }
            _ => {
                return self.error("Invalid literals/variables in Zip Function arguments.".to_string())
            }
        };
        
        self.consume(Token::RightParen, "Expect ')' after Zip Function.".to_string())?; 
        Ok(JsonValue::Arr(res))
    }

    fn partition(&mut self, a: &mut [JsonValue], temp_1: &(JsonValue, Option<JsonValue>), temp_2: &(JsonValue, Option<JsonValue>)) -> Result<usize, String> {
        let mut i = 0;
        let right = a.len() - 1;
     
        for j in 0..right {
            if let JsonValue::Str(ref s) = &temp_1.0 {
                self.locals[self.depth as usize].insert(s.clone(), 
                    a[j].clone()
                );
            }
            
            if let JsonValue::Str(ref s) = &temp_2.0 {
                self.locals[self.depth as usize].insert(s.clone(), 
                    a[right].clone()
                );
            }
            
            let goto_point = self.current;
            let condition = self.val_expression()?;
            
            match condition {
                JsonValue::Bool(b) => {
                    if b {
                        a.swap(j, i);
                        i += 1;
                    }
                }
                v => {
                    return self.error(format!("Sort condition must be boolean got {:?}.", v));
                }
            }
            
            self.current = goto_point;
        }
     
        a.swap(i, right);
        Ok(i)
    }
     
    fn quicksort(&mut self, a: &mut [JsonValue], temp_1: &(JsonValue, Option<JsonValue>), temp_2: &(JsonValue, Option<JsonValue>)) -> Result<(), String> {
        if a.len() > 1 {
            let q = self.partition(a, temp_1, temp_2)?;
            self.quicksort(&mut a[..q], temp_1, temp_2)?;
            self.quicksort(&mut a[q+1..], temp_1, temp_2)?;
        }

        Ok(())
    }
    
    fn sort_impl(&mut self) -> Result<JsonValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Sort Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            JsonValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                self.consume(Token::Pipe, "Expect '|' before args.".to_string())?;
                let temp_1 = self.create_temp_var()?;
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                let temp_2 = self.create_temp_var()?;
                self.consume(Token::Pipe, "Expect '|' after args.".to_string())?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;

                let mut new_vec = a.clone();

                self.quicksort(&mut new_vec[..], &temp_1, &temp_2)?;
                self.val_expression()?;
                
                self.restore_temp_var(temp_1);
                self.restore_temp_var(temp_2);
                
                self.consume(Token::RightParen, "Expect ')' after Sort Function.".to_string())?; 
                Ok(JsonValue::Arr(new_vec))
            }
            _ => {
                self.error("Invalid literal/variable in Sort Function arguments.".to_string())
            }
        }
    }
    
    fn primary(&mut self) -> Result<JsonValue, String> {
        match self.peek().clone() {
            Token::Fn(func) => {
                match func {
                    Function::Filter => {
                        self.filter_impl()
                    }
                    Function::Map => {
                        self.map_impl()
                    }
                    Function::Chars => {
                        self.chars_impl()
                    }
                    Function::String => {
                        self.string_impl()
                    }
                    Function::Keys => {
                        self.keys_impl()
                    }
                    Function::Values => {
                        self.values_impl()
                    }
                    Function::Fold => {
                        self.fold_impl()
                    }
                    Function::Zip => {
                        self.zip_impl()
                    }
                    Function::Sort => {
                        self.sort_impl()
                    }
                }
            }
            Token::Num(n) => {
                self.eat();
                Ok(JsonValue::Num(n))
            }
            Token::Bool(b) => {
                self.eat();
                Ok(JsonValue::Bool(b))
            }
            Token::Str(s) => {
                self.eat();
                Ok(JsonValue::Str(s))
            }
            Token::Nil => {
                self.eat();
                Ok(JsonValue::Nil) 
            }
            Token::LeftCurl => {
                self.eat();
                self.locals.push(HashMap::new());
                self.depth += 1;
                let obj = self.block()?;
                self.depth -= 1;
                if self.depth == -1 {
                    self.locals = vec!();
                }
                Ok(obj)
            }
            Token::LeftBracket => {
                self.eat();
                let obj = self.array()?;
                Ok(obj)
            }
            Token::Var(v) => {
                self.eat();
                
                if self.depth-v.0 < 0 {
                    Ok(JsonValue::Nil)
                } else {
                    let value = match self.locals[(self.depth-v.0) as usize].get(&v.1) {
                        Some(JsonValue::Num(n)) => {
                            Ok(JsonValue::Num(*n))
                        }
                        Some(JsonValue::Str(s)) => {
                            Ok(JsonValue::Str(s.to_string()))
                        }
                        Some(JsonValue::Bool(b)) => {
                            Ok(JsonValue::Bool(*b))
                        }
                        Some(JsonValue::Obj(o)) => {
                            Ok(JsonValue::Obj(o.clone()))
                        }
                        Some(JsonValue::Arr(a)) => {
                            Ok(JsonValue::Arr(a.clone()))
                        }
                        _ => {
                            Ok(JsonValue::Nil)
                        }
                    };
                    
                    match value {
                        Ok(JsonValue::Obj(o)) => {
                            let mut c = JsonValue::Obj(o.clone());
                        
                            let mut call_type = self.peek().clone();

                            loop {
                                
                                call_type = if let Token::LeftBracket | Token::Dot = call_type {
                                    self.peek().clone()
                                } else {
                                    break;
                                };

                                match call_type {
                                    Token::Dot => self.consume(Token::Dot, "Expect '.' after Object Variable.".to_string())?,
                                    Token::LeftBracket => self.consume(Token::LeftBracket, "Expect '[' after Object Variable.".to_string())?,
                                    _ => break
                                };

                                match self.peek().clone() {
                                    Token::Num(_) | Token::Var(_) | Token::Str(_) | Token::LeftParen 
                                    | Token::Bool(_) | Token::Not | Token::Sharp | Token::LeftBracket
                                    | Token::LeftCurl | Token::Nil | Token::Fn(_) => {
                                        let val = match call_type {
                                            Token::LeftBracket => self.val_expression()?,
                                            Token::Dot => self.primary()?,
                                            _ => self.primary()?
                                        };
                                        
                                        match val {
                                            JsonValue::Str(s) => {
                                                match c.clone() {
                                                    JsonValue::Obj(oo) => {
                                                        if let Some(v) = oo.data.get(&s) {
                                                            c = v.clone();
                                                        } else {
                                                            c = JsonValue::Nil;
                                                        }
                                                    }
                                                    _ => {
                                                        return self.error("Expect key after selecter.".to_string())
                                                    }
                                                }
                                            }
                                            JsonValue::Num(n) => {
                                                match c.clone() {
                                                    JsonValue::Arr(aa) => {
                                                        if n < 0.0 {
                                                            c = JsonValue::Nil
                                                        } else {
                                                            if let Some(v) = aa.get(n as usize) {
                                                                c = v.clone()
                                                            } else {
                                                                c = JsonValue::Nil
                                                            }
                                                        }
                                                    }
                                                    JsonValue::Str(ss) => {
                                                        if n < 0.0 {
                                                            c = JsonValue::Nil
                                                        } else {
                                                            if let Some(v) = ss.chars().nth(n as usize) {
                                                                c = JsonValue::Str(v.to_string())
                                                            } else {
                                                                c = JsonValue::Nil
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        return self.error("Expect key after selecter.".to_string())
                                                    }
                                                }
                                            }
                                            _ => { 
                                                return self.error("Expect key after selecter.".to_string())
                                            }
                                        }
                                        if let Token::LeftBracket = call_type {
                                            self.consume(Token::RightBracket, "Expect ']' after Array Variable.".to_string())?;
                                        }
                                    }
                                    _ => { }
                                }
                            }
                            
                            Ok(c)
                        }
                        Ok(JsonValue::Arr(a)) => {
                            let mut c = JsonValue::Arr(a.clone());
                            
                            let mut call_type = self.peek().clone();
                            
                            loop {
                                call_type = if let Token::LeftBracket | Token::Dot = call_type {
                                    self.peek().clone()
                                } else {
                                    break;
                                };
                                
                                match call_type {
                                    Token::Dot => self.consume(Token::Dot, "Expect '.' after Object Variable.".to_string())?,
                                    Token::LeftBracket => self.consume(Token::LeftBracket, "Expect '[' after Object Variable.".to_string())?,
                                    _ => break
                                };
                                
                                match self.peek().clone() {
                                    Token::Num(_) | Token::Var(_) | Token::Str(_) | Token::LeftParen 
                                    | Token::Bool(_) | Token::Not | Token::Sharp | Token::LeftBracket
                                    | Token::LeftCurl | Token::Nil => {
                                        let val = match call_type {
                                            Token::LeftBracket => self.val_expression()?,
                                            Token::Dot => self.primary()?,
                                            _ => self.primary()?
                                        };
                                        
                                        match val {
                                            JsonValue::Str(s) => {
                                                match c.clone() {
                                                    JsonValue::Obj(oo) => {
                                                        if let Some(v) = oo.data.get(&s) {
                                                            c = v.clone();
                                                        } else {
                                                            c = JsonValue::Nil;
                                                        }
                                                    }
                                                    _ => {
                                                        return self.error("Expect index after selecter.".to_string())
                                                    }
                                                }
                                            }
                                            JsonValue::Num(n) => {
                                                match c.clone() {
                                                    JsonValue::Arr(aa) => {
                                                        if n < 0.0 {
                                                            c = JsonValue::Nil
                                                        } else {
                                                            if let Some(v) = aa.get(n as usize) {
                                                                c = v.clone()
                                                            } else {
                                                                c = JsonValue::Nil
                                                            }
                                                        }
                                                    }
                                                    JsonValue::Str(ss) => {
                                                        if n < 0.0 {
                                                            c = JsonValue::Nil
                                                        } else {
                                                            if let Some(v) = ss.chars().nth(n as usize) {
                                                                c = JsonValue::Str(v.to_string())
                                                            } else {
                                                                c = JsonValue::Nil
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        return self.error("Expect index after selecter.".to_string())
                                                    }
                                                }
                                            }
                                            _ => {
                                                return self.error("Expect index after selecter.".to_string())
                                            }
                                        }
                                        
                                        if let Token::LeftBracket = call_type {
                                            self.consume(Token::RightBracket, "Expect ']' after Array Variable.".to_string())?;
                                        }
                                    }
                                    _ => { }
                                }
                            }
                            
                            Ok(c)
                        }
                        Ok(JsonValue::Str(s)) => {
                            match self.peek() {
                                Token::Dot => {
                                    self.eat();
                                    let expr = self.val_expression()?;
                                    match expr {
                                        JsonValue::Num(n) => {
                                            if n < 0.0 {
                                                Ok(JsonValue::Nil)
                                            } else {
                                                if let Some(v) = s.chars().nth(n as usize) {
                                                    Ok(JsonValue::Str(v.to_string()))
                                                } else {
                                                    Ok(JsonValue::Nil)
                                                }
                                            }
                                        }
                                        _ => {
                                            self.error("Expect index after '.'".to_string())
                                        }
                                    }
                                }
                                Token::LeftBracket => {
                                    self.eat();
                                    let expr = self.val_expression()?;
                                    match expr {
                                        JsonValue::Num(n) => {  
                                            self.consume(Token::RightBracket, "Expect ']' after String Variable.".to_string())?;
                                            if n < 0.0 {
                                                Ok(JsonValue::Nil)
                                            } else {
                                                if let Some(v) = s.chars().nth(n as usize) {
                                                    Ok(JsonValue::Str(v.to_string()))
                                                } else {
                                                    Ok(JsonValue::Nil)
                                                }
                                            }
                                            
                                        }
                                        _ => {
                                            self.error("Expect index after '['".to_string())
                                        }
                                    }
                                }
                                _ => {
                                    Ok(JsonValue::Str(s.clone()))
                                }
                            }
                        }
                        _ => {
                            value
                        }
                    }
                }
            }
            Token::LeftParen => {
                self.eat();
                let r = self.val_expression();
                self.consume(Token::RightParen, "Expect ')'.".to_string())?;
                r
            }
            _ => { 
                Ok(JsonValue::Nil) 
            }
        }    
    }
    
    fn val_expression(&mut self) -> Result<JsonValue, String> {
        self.equality()
    }
    
    fn array_value(&mut self) -> Result<JsonValue, String> {
        match self.peek().clone() {
            Token::Num(_) | Token::Var(_) | Token::Str(_) | Token::LeftParen 
            | Token::Bool(_) | Token::Not | Token::Sharp | Token::LeftBracket
            | Token::LeftCurl | Token::Nil | Token::Fn(_) => {
                Ok(self.val_expression()?)
            }
            _ => { self.error("Expected Value.".to_string()) }
        }
    }
    
    fn array(&mut self) -> Result<JsonValue, String> {
        let mut result = vec!();
        
        while !self.check(Token::RightBracket) && !self.at_end() {
            let val = self.array_value()?;
            result.push(val);
            if !self.check(Token::RightBracket) {
                //if self.peek_full().line == self.prev_full().line {
                    self.consume(Token::Comma, "Expect ','.".to_string())?;  
                //}
            }
        }
        
        self.consume(Token::RightBracket, "Expect ']' after array.".to_string())?;
        Ok(JsonValue::Arr(result))
    }
    
    fn key(&mut self) -> Result<(String, JsonValue), String> {
        match self.peek().clone() {
            Token::Str(s) => {
                self.eat();
                self.consume(Token::Colon, "Expected ':' after Key identifier".to_string())?;
                
                let result = match self.peek().clone() {
                    Token::Num(_) | Token::Var(_) | Token::Str(_) | Token::LeftParen 
                    | Token::Bool(_) | Token::Not | Token::Sharp | Token::LeftBracket
                    | Token::LeftCurl | Token::Nil | Token::Fn(_) => {
                        Ok((s.clone(), self.val_expression()?))
                    }
                    _ => { self.error("Expected Key: Value pair.".to_string()) }
                };
    
                result
            }
            Token::Var(_) => {
                let v_key = self.primary()?;
                
                if let JsonValue::Str(s) = v_key {
                    self.consume(Token::Colon, "Expected ':' after Key identifier".to_string())?;
                    
                    let result = match self.peek().clone() {
                        Token::Num(_) | Token::Var(_) | Token::Str(_) | Token::LeftParen 
                        | Token::Bool(_) | Token::Not | Token::Sharp | Token::LeftBracket
                        | Token::LeftCurl | Token::Nil | Token::Fn(_) => {
                            Ok((s.clone(), self.val_expression()?))
                        }
                        _ => { self.error("Expected Key: Value pair.".to_string()) }
                    };
        
                    result
                } else {
                    self.error("Expected Key.".to_string())
                }
            }
            _ => {
                self.error("Expected Key.".to_string())
            }
        }
    }
    
    fn expression(&mut self) -> Result<(String, JsonValue), String> {
        if let Token::Str(_) | Token::Var(_) = self.peek() {
            self.key()
        } else {
            self.error("Expression Error.".to_string())
        }
    }
    
    fn object(&mut self) -> Result<JsonValue, String> {
        if self.match_single(Token::LeftCurl) {
            self.locals.push(HashMap::new());
            self.depth += 1;
            self.block()
        } else {
            self.error("Object Error.".to_string())
        }
    }
    
    fn block(&mut self) -> Result<JsonValue, String> {
        let mut result = JsonObj::new();
        
        while !self.check(Token::RightCurl) && !self.at_end() {
            let key_val = self.expression()?;
            if let Some(_) = result.data.get(&key_val.0) {
                return self.error("Duplicate Key.".to_string());
            } else {
                result.data.insert(key_val.0.clone(), key_val.1.clone());
                self.locals[self.depth as usize].insert(key_val.0, key_val.1);
            }
            if !self.check(Token::RightCurl) {
                //if self.peek_full().line == self.prev_full().line {
                    self.consume(Token::Comma, "Expect ','.".to_string())?;  
                //}
            }
        }
        self.consume(Token::RightCurl, "Expect '}' after block.".to_string())?;
        Ok(JsonValue::Obj(result))
    }

    pub fn parse(&mut self, lexer: &mut JsonLexer, debug: bool) -> Result<Json, String> {
        if debug {  
            println!("----Src----"); 
            println!("{}", lexer.source);
            println!("----Lex----");
        }
        
        let now = Instant::now();

        loop {
            match lexer.scan() {
                Ok(TokenData {
                    token: Token::EOF,
                    line: _,
                    section: _
                }) => {
                    break;
                }
                Ok(TokenData {
                    token: Token::Macro(mac),
                    line: _,
                    section: _
                }) => {
                    for i in mac.into_iter() {
                        if debug { println!("{}", i); }
                        self.tokens.push(i);
                    }
                }
                Ok(token) => {
                    if debug { println!("{}", token); }
                    self.tokens.push(token);
                }
                Err(msg) => {
                    if msg == "Macro" {
                        continue;
                    }
                    return Err(format!("{}", msg));
                }
            }
        }
        
        
        if debug { 
            println!("[Completed in {} ms]", now.elapsed().as_millis());
            println!("----Parse----"); 
        }
        let mut result = vec!();
        
        while !self.at_end() {
            match self.object() {
                Ok(value) => result.push(value),
                Err(e) => return Err(e)
            }
            
            if !self.match_single(Token::Comma) {
                if self.check(Token::LeftCurl) {
                    //if self.peek_full().line == self.prev_full().line {
                        return self.error("Expect ','.".to_string())
                    //}
                }
                
            }
        }
        let final_result = Json { data: result };

        if debug { 
            println!("[Completed in {} ms]", now.elapsed().as_millis()); 
            println!("{}", &final_result);
        }

        Ok(final_result)
    }
}