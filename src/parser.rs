use std::{collections::HashMap, time::Instant};

use crate::{lexer::{Function, EconLexer, Token, TokenData}, object::EconObj, value::EconValue};

pub struct EconParser { 
    tokens: Vec<TokenData>,
    current: usize,
    source: String,
    locals: Vec<HashMap<String, EconValue>>,
    constraints: Vec<HashMap<String, Vec<(usize, bool)>>>,
    depth: isize,
    in_constraint: bool
}

impl EconParser {
    pub fn new(src: &str) -> Self {
        Self {
            tokens: vec!(),
            current: 0,
            source: String::from(src),
            locals: vec!(),
            constraints: vec!(),
            depth: -1,
            in_constraint: false
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

        for (line_num, line) in self.source.lines().enumerate() {
            if line_num+1 == current_line || (line_num != 0 && line_num-1 == current_line) {
                result_err.push_str(&format!("[{:04}]   {}\n", line_num, line));
            }
            if line_num == current_line {
                result_err.push_str(&format!("-> [{:04}]{}\n", line_num, line));
            }
        }
        
        Err(result_err)
    }

    fn error_at<T>(&self, msg: String, current_line: usize) -> Result<T, String> {
        let mut result_err = String::from("");
        
        result_err.push_str(&format!("Line [{:04}] Error Parsing -> \"{}\"\n", current_line, msg.clone()));

        for (line_num, line) in self.source.lines().enumerate() {
            if line_num+1 == current_line || (line_num != 0 && line_num-1 == current_line) {
                result_err.push_str(&format!("[{:04}]   {}\n", line_num, line));
            }
            if line_num == current_line {
                result_err.push_str(&format!("-> [{:04}]{}\n", line_num, line));
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
    
    fn equality(&mut self) -> Result<EconValue, String> {
        let mut left = self.comparison()?;
        
        loop {
            match self.peek() {
                Token::Equal => {
                    self.eat();
                    let right = self.comparison()?;

                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Bool(n1==n2)
                        }
                        (&EconValue::Bool(ref n1), &EconValue::Bool(ref n2)) => {
                            EconValue::Bool(n1==n2)
                        }
                        (&EconValue::Str(ref n1), &EconValue::Str(ref n2)) => {
                            EconValue::Bool(n1==n2)
                        }
                        _ => return self.error(format!("Invalid '==' of types: {}+{}", left, right))
                    };
                }
                Token::NotEqual => {
                    self.eat();
                    let right = self.comparison()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Bool(n1!=n2)
                        }
                        (&EconValue::Bool(ref n1), &EconValue::Bool(ref n2)) => {
                            EconValue::Bool(n1!=n2)
                        }
                        (&EconValue::Str(ref n1), &EconValue::Str(ref n2)) => {
                            EconValue::Bool(n1!=n2)
                        }
                        _ => return self.error(format!("Invalid '~=' of types: {}+{}", left, right))
                    };
                }
                Token::Question => {
                    self.eat();
                    let right1 = self.equality()?;
                    self.consume(Token::Colon, "Expect ':'.".to_string())?;
                    let right2 = self.equality()?;
                    
                    left = match &left {
                        &EconValue::Bool(true) => {
                            right1
                        }
                        &EconValue::Bool(false) => {
                            right2
                        }
                        _ => return self.error(format!("Invalid ternary expected bool got: {}", left))
                    };
                }
                _ => { break; }
            }
        }
        
        Ok(left)
    }
    
    fn comparison(&mut self) -> Result<EconValue, String> {
        let mut left = self.term()?;
        
        loop {
            match self.peek() {
                Token::Less => {
                    self.eat();
                    let right = self.term()?;
                    
                    left = match (&left, &right) {
                        (EconValue::Num(n1), EconValue::Num(n2)) => {
                            EconValue::Bool(*n1 < *n2)
                        }
                        (EconValue::Str(n1), EconValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = c2 as u32 - 48 > c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            EconValue::Bool(res)
                        }
                        _ => return self.error(format!("Invalid '<' of types: {}+{}", left, right))
                    };
                }
                Token::Greater => {
                    self.eat();
                    let right = self.term()?;
                    left = match (&left, &right) {
                        (EconValue::Num(n1), EconValue::Num(n2)) => {
                            EconValue::Bool(*n1 > *n2)
                        }
                        (EconValue::Str(n1), EconValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = (c2 as u32 - 48) < c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            EconValue::Bool(res)
                        }
                        _ => return self.error(format!("Invalid '>' of types: {}+{}", left, right))
                    };
                }
                Token::GreaterEqual => {
                    self.eat();
                    let right = self.term()?;
                    left = match (&left, &right) {
                        (EconValue::Num(n1), EconValue::Num(n2)) => {
                            EconValue::Bool(*n1 >= *n2)
                        }
                        (EconValue::Str(n1), EconValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = (c2 as u32 - 48) < c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            EconValue::Bool(res)
                        }
                        _ => return self.error(format!("Invalid '>=' of types: {}+{}", left, right))
                    };
                }
                Token::LessEqual => {
                    self.eat();
                    let right = self.term()?;
                    left = match (&left, &right) {
                        (EconValue::Num(n1), EconValue::Num(n2)) => {
                            EconValue::Bool(*n1 <= *n2)
                        }
                        (EconValue::Str(n1), EconValue::Str(n2)) => {
                            let mut  res: bool = true;
                            for (i, c) in n1.chars().enumerate() {
                                if let Some(c2) = n2.chars().nth(i) {
                                    res = c2 as u32 - 48 > c as u32 - 48;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        
                            EconValue::Bool(res)
                        }
                        _ => return self.error(format!("Invalid '<=' of types: {}+{}", left, right))
                    };
                }
                Token::And => {
                    self.eat();
                    let right = self.term()?;
                    if let (EconValue::Bool(n1), EconValue::Bool(n2)) = (&left, right) {
                        left = EconValue::Bool(*n1 && n2);
                    } 
                }
                Token::Or => {
                    self.eat();
                    let right = self.term()?;
                    if let (EconValue::Bool(n1), EconValue::Bool(n2)) = (&left, right) {
                        left = EconValue::Bool(*n1 || n2);
                    } 
                }
                _ => { break; }
            }
        }
        
        Ok(left)
    }
    
    fn term(&mut self) -> Result<EconValue, String> {
        let mut left = self.factor()?;
        
        loop {
            match self.peek() {
                Token::Plus => {
                    self.eat();
                    let right = self.factor()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Num(n1+n2)
                        }
                        (&EconValue::Str(ref n1), &EconValue::Str(ref n2)) =>  {
                            EconValue::Str(format!("{}{}", n1, n2))
                        }
                        (&EconValue::Str(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Str(format!("{}{}", n1, n2))
                        }
                        (&EconValue::Num(ref n1), &EconValue::Str(ref n2)) =>  {
                            EconValue::Str(format!("{}{}", n1, n2))
                        }
                        (&EconValue::Num(ref n1), &EconValue::Nil) =>  {
                            EconValue::Num(*n1)
                        }
                        (&EconValue::Nil, &EconValue::Num(ref n1)) =>  {
                            EconValue::Num(*n1)
                        }
                        (&EconValue::Str(ref n1), &EconValue::Nil) =>  {
                            EconValue::Str(format!("{}", n1))
                        }
                        (&EconValue::Nil, &EconValue::Str(ref n1)) =>  {
                            EconValue::Str(format!("{}", n1))
                        }
                        (&EconValue::Str(ref n1), &EconValue::Bool(ref n2)) =>  {
                            EconValue::Str(format!("{}{}", n1, n2))
                        }
                        (&EconValue::Bool(ref n1), &EconValue::Str(ref n2)) =>  {
                            EconValue::Str(format!("{}{}", n1, n2))
                        }
                        (&EconValue::Arr(ref n1), &EconValue::Arr(ref n2)) =>  {
                            let mut new_arr = vec!();
                            
                            for a in n1 {
                                new_arr.push(a.clone());
                            }
                            for a in n2 {
                                new_arr.push(a.clone());
                            }

                            EconValue::Arr(new_arr)
                        }
                        (&EconValue::Arr(ref n1), &EconValue::Nil) =>  {
                            let mut new_arr = vec!();
                            
                            for a in n1 {
                                new_arr.push(a.clone());
                            }

                            EconValue::Arr(new_arr)
                        }
                        (&EconValue::Nil, &EconValue::Arr(ref n1)) =>  {
                            let mut new_arr = vec!();
                            
                            for a in n1 {
                                new_arr.push(a.clone());
                            }

                            EconValue::Arr(new_arr)
                        }
                        (&EconValue::Obj(ref n1), &EconValue::Obj(ref n2)) =>  {
                            let mut new_obj = EconObj::new();
                            
                            for (k, v) in &n1.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }
                            for (k, v) in &n2.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }

                            EconValue::Obj(new_obj)
                        }
                        (&EconValue::Obj(ref n1), &EconValue::Nil) =>  {
                            let mut new_obj = EconObj::new();
                            
                            for (k, v) in &n1.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }
                            
                            EconValue::Obj(new_obj)
                        }
                        (&EconValue::Nil, &EconValue::Obj(ref n1)) =>  {
                            let mut new_obj = EconObj::new();
                            
                            for (k, v) in &n1.data {
                                new_obj.data.insert(k.clone(), v.clone());
                            }
                            
                            EconValue::Obj(new_obj)
                        }
                        _ => return self.error(format!("Invalid addition of types: {}+{}", left, right))
                    };
                }
                Token::BackSlash => {
                    self.eat();
                    let right = self.factor()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Str(ref n1), &EconValue::Str(ref n2)) =>  {
                            EconValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&EconValue::Str(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&EconValue::Num(ref n1), &EconValue::Str(ref n2)) =>  {
                            EconValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&EconValue::Str(ref n1), &EconValue::Nil) =>  {
                            EconValue::Str(format!("{}", n1))
                        }
                        (&EconValue::Nil, &EconValue::Str(ref n1)) =>  {
                            EconValue::Str(format!("{}", n1))
                        }
                        (&EconValue::Str(ref n1), &EconValue::Bool(ref n2)) =>  {
                            EconValue::Str(format!("{}\n{}", n1, n2))
                        }
                        (&EconValue::Bool(ref n1), &EconValue::Str(ref n2)) =>  {
                            EconValue::Str(format!("{}\n{}", n1, n2))
                        }
                        _ => return self.error(format!("Invalid concatenation of types: {}+{}", left, right))
                    };
                }
                Token::Minus => {
                    self.eat();
                    let right = self.factor()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Num(n1-n2)
                        }
                        _ => return self.error(format!("Invalid subtraction of types: {}+{}", left, right))
                    };
                }
                _ => {
                    break;
                }
            }
        }
        
        Ok(left)
    }
    
    fn factor(&mut self) -> Result<EconValue, String> {
        let mut left = self.unary()?;
        
        loop {
            match self.peek() {
                Token::Mult => {
                    self.eat();
                    let right = self.unary()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Num(n1*n2)
                        }
                        _ => return self.error(format!("Invalid '*' of types: {}+{}", left, right))
                    };
                }
                Token::Div => {
                    self.eat();
                    let right = self.unary()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Num(n1/n2)
                        }
                        _ => return self.error(format!("Invalid '/' of types: {}+{}", left, right))
                    };
                }
                Token::Percent => {
                    self.eat();
                    let right = self.unary()?;
                    
                    left = match (&left, &right) {
                        (&EconValue::Num(ref n1), &EconValue::Num(ref n2)) =>  {
                            EconValue::Num(n1.rem_euclid(*n2))
                        }
                        _ => return self.error(format!("Invalid '%' of types: {}+{}", left, right))
                    };
                }
                _ => {
                    break;
                }
            }
        }
        
        Ok(left)
    }
    
    fn unary(&mut self) -> Result<EconValue, String> {
        match self.peek().clone() {
            Token::Minus => {
                self.eat();
                let right = self.unary()?;
                if let EconValue::Num(n1) = right {
                    Ok(EconValue::Num(-n1))
                } else {
                    Ok(right)
                }
            }
            Token::Not => {
                self.eat();
                let right = self.unary()?;
                if let EconValue::Bool(b1) = right {
                    Ok(EconValue::Bool(!b1))
                } else {
                    Ok(right)
                }
            }
            Token::Sharp => {
                self.eat();
                let right = self.unary()?;
                match &right {
                    &EconValue::Str(ref n1) => {
                        Ok(EconValue::Num(n1.chars().count() as f64))
                    }
                    &EconValue::Num(ref n1) => {
                        Ok(EconValue::Num(*n1))
                    }
                    &EconValue::Arr(ref n1) => {
                        Ok(EconValue::Num(n1.len() as f64))
                    }
                    &EconValue::Obj(ref n1) => {
                        Ok(EconValue::Num(n1.data.keys().len() as f64))
                    }
                    _ => return self.error(format!("Invalid '#' of type: {}", right))
                }
            }
            _ => {
                self.primary()
            }
        }
    }
    
    fn filter_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            EconValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_vec = vec!();
                for (j, aa) in a.iter().enumerate() {
                    if let EconValue::Str(ref s) = &temp_1.0 {
                        self.locals[self.depth as usize].insert(s.clone(), aa.clone());
                    } 
                    let goto_point = self.current;
                    let condition = self.val_expression()?;
                    
                    match condition {
                        EconValue::Bool(true) => {
                            new_vec.push(aa.clone());
                        }
                        EconValue::Bool(false) => { }
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
                Ok(EconValue::Arr(new_vec))
            }
            EconValue::Obj(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_obj = EconObj::new();
                for (j, aa) in a.data.iter().enumerate() {
                    if let EconValue::Str(ref s) = &temp_1.0 {
                        let mut key_val = EconObj::new();
                        key_val.data.insert("key".to_string(), EconValue::Str(aa.0.clone()));
                        key_val.data.insert("val".to_string(), aa.1.clone());
                        self.locals[self.depth as usize].insert(s.clone(), 
                            EconValue::Obj(key_val)
                        );
                    } 
                    let goto_point = self.current;
                    let condition = self.val_expression()?;
                    
                    match condition {
                        EconValue::Bool(true) => {
                            new_obj.data.insert(aa.0.clone(), aa.1.clone());
                        }
                        EconValue::Bool(false) => { }
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
                Ok(EconValue::Obj(new_obj))
            }
            _ => {
                return self.error("Invalid literal/variable in function arguments.".to_string());
            }
        }
    }
    
    fn keys_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Keys Function.".to_string())?;
        let right = self.val_expression()?;
        
        if let EconValue::Obj(s) = right {
            self.consume(Token::RightParen, "Expect ')' after Keys Function.".to_string())?; 
            let mut new_vec = vec!();
            for i in s.data.keys() {
                new_vec.push(EconValue::Str(i.to_string()));
            }
            Ok(EconValue::Arr(new_vec))
        } else {
            return self.error("Invalid literal/variable in Keys Function arguments.".to_string());
        }
    }
    
    fn values_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Keys Function.".to_string())?;
        let right = self.val_expression()?;
        
        if let EconValue::Obj(s) = right {
            self.consume(Token::RightParen, "Expect ')' after Keys Function.".to_string())?; 
            let mut new_vec = vec!();
            for i in s.data.values() {
                new_vec.push(i.clone());
            }
            Ok(EconValue::Arr(new_vec))
        } else {
            return self.error("Invalid literal/variable in Keys Function arguments.".to_string());
        }
    }
    
    fn chars_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Chars Function.".to_string())?;
        let right = self.val_expression()?;
        
        if let EconValue::Str(s) = right {
            self.consume(Token::RightParen, "Expect ')' after Chars Function.".to_string())?; 
            let mut new_vec = vec!();
            for i in s.chars() {
                new_vec.push(EconValue::Str(i.to_string()));
            }
            Ok(EconValue::Arr(new_vec))
        } else {
            return self.error("Invalid literal/variable in Chars Function arguments.".to_string());
        }
    }
    
    fn string_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after String Function.".to_string())?;
        let right = self.val_expression()?;

        if let &EconValue::Arr(ref a) = &right {
            self.consume(Token::RightParen, "Expect ')' after String Function.".to_string())?; 
            let mut new_str = String::from("");
            for i in a {
                if let EconValue::Str(s) = i {
                    new_str.push_str(s);
                }
            }
            Ok(EconValue::Str(new_str))
        } else {
            return self.error("Invalid literal/variable in String Function arguments.".to_string());
        }
    }
    
    fn map_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Map Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            EconValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_vec = vec!();
                for (j, aa) in a.iter().enumerate() {
                    if let EconValue::Str(ref s) = &temp_1.0 {
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
                Ok(EconValue::Arr(new_vec))
            }
            EconValue::Obj(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                let temp_1 = self.create_temp_var()?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                let mut new_obj = EconObj::new();
                for (j, aa) in a.data.iter().enumerate() {
                    if let EconValue::Str(ref s) = &temp_1.0 {
                        let mut key_val = EconObj::new();
                        key_val.data.insert("key".to_string(), EconValue::Str(aa.0.clone()));
                        key_val.data.insert("val".to_string(), aa.1.clone());
                        self.locals[self.depth as usize].insert(s.clone(), 
                            EconValue::Obj(key_val)
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
                Ok(EconValue::Obj(new_obj))
            }
            _ => {
                self.error("Invalid literal/variable in Map Function arguments.".to_string())
            }
        }
    }
    
    fn create_temp_var(&mut self) -> Result<(EconValue, Option<EconValue>), String> {
        let i_name = self.val_expression()?;
        let cached_val;
                
        if let EconValue::Str(ref s) = &i_name {
            cached_val = if let Some(v) = self.locals[self.depth as usize].get(s) {
                Some(v.clone())
            } else {
                None
            };
            self.locals[self.depth as usize].insert(s.clone(), EconValue::Nil);
        } else {
            return self.error("Invalid iterator identifier.".to_string())
        }
        
        Ok((i_name.clone(), cached_val.clone()))
    }
    
    fn restore_temp_var(&mut self, var: (EconValue, Option<EconValue>)) {
        if let (EconValue::Str(ref s), Some(cache)) = (&var.0, var.1) {
            self.locals[self.depth as usize].insert(s.clone(), cache.clone());
        }
    }
    
    fn fold_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Fold Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            EconValue::Arr(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                self.consume(Token::Pipe, "Expect '|' before args.".to_string())?;
                let temp_1 = self.create_temp_var()?;
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                let temp_2 = self.create_temp_var()?;
                self.consume(Token::Pipe, "Expect '|' after args.".to_string())?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;

                for (j, aa) in a.iter().enumerate() {
                    if let EconValue::Str(ref s) = &temp_1.0 {
                        self.locals[self.depth as usize].insert(s.clone(), 
                            aa.clone()
                        );
                    }
                    
                    if let EconValue::Str(ref s) = &temp_2.0 {
                        if let None = self.locals[self.depth as usize].get(s) {
                            self.locals[self.depth as usize].insert(s.clone(), EconValue::Nil);
                        }
                    }
                    
                    let goto_point = self.current;
                    let expr = self.val_expression()?;
                    
                    if let EconValue::Str(ref s) = &temp_2.0 {
                        self.locals[self.depth as usize].insert(s.clone(), expr);
                    }

                    if j < a.len()-1 {
                        self.current = goto_point;
                    }
                }
                
                let ret_val = if let EconValue::Str(ref s) = &temp_2.0 {
                    if let Some(rv) = self.locals[self.depth as usize].get(s) {
                        rv.clone()
                    } else {
                        EconValue::Nil
                    }
                } else {
                    EconValue::Nil
                };
                
                self.restore_temp_var(temp_1);
                self.restore_temp_var(temp_2);
                
                self.consume(Token::RightParen, "Expect ')' after Fold Function.".to_string())?; 
                Ok(ret_val)
            }
            EconValue::Obj(a) => {
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                
                self.consume(Token::Pipe, "Expect '|' before args.".to_string())?;
                let temp_1 = self.create_temp_var()?;
                self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
                let temp_2 = self.create_temp_var()?;
                self.consume(Token::Pipe, "Expect '|' after args.".to_string())?;
                
                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;

                for (j, aa) in a.data.iter().enumerate() {
                    if let EconValue::Str(ref s) = &temp_1.0 {
                        let mut key_val = EconObj::new();

                        key_val.data.insert("key".to_string(), EconValue::Str(aa.0.clone()));
                        key_val.data.insert("val".to_string(), aa.1.clone());
                        self.locals[self.depth as usize].insert(s.clone(), 
                            EconValue::Obj(key_val)
                        );
                    }
                    
                    if let EconValue::Str(ref s) = &temp_2.0 {
                        if let None = self.locals[self.depth as usize].get(s) {
                            self.locals[self.depth as usize].insert(s.clone(), EconValue::Nil);
                        }
                    }
                    
                    let goto_point = self.current;
                    let expr = self.val_expression()?;
                    
                    if let EconValue::Str(ref s) = &temp_2.0 {
                        self.locals[self.depth as usize].insert(s.clone(), expr);
                    }

                    if j < a.data.keys().len()-1 {
                        self.current = goto_point;
                    }
                }
                
                let ret_val = if let EconValue::Str(ref s) = &temp_2.0 {
                    if let Some(rv) = self.locals[self.depth as usize].get(s) {
                        rv.clone()
                    } else {
                        EconValue::Nil
                    }
                } else {
                    EconValue::Nil
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

    fn zip_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Zip Function.".to_string())?;
        let a = self.val_expression()?;
        self.consume(Token::Comma, "Expect ',' after arg.".to_string())?;
        let b = self.val_expression()?;
        
        let res = match (a, b) {
            (EconValue::Arr(aa), EconValue::Arr(bb)) => {
                let mut i = 0;
                let mut ret = vec!();
                loop {
                    match (aa.get(i), bb.get(i)) {
                        (Some(av), Some(bv)) => {
                            ret.push(EconValue::Arr(vec![av.clone(), bv.clone()]))
                        }
                        (Some(av), None) => {
                            ret.push(EconValue::Arr(vec![av.clone(), EconValue::Nil]))
                        }
                        (None, Some(bv)) => {
                            ret.push(EconValue::Arr(vec![EconValue::Nil, bv.clone()]))
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
        Ok(EconValue::Arr(res))
    }

    fn partition(&mut self, a: &mut [EconValue], temp_1: &(EconValue, Option<EconValue>), temp_2: &(EconValue, Option<EconValue>)) -> Result<usize, String> {
        let mut i = 0;
        let right = a.len() - 1;
     
        for j in 0..right {
            if let EconValue::Str(ref s) = &temp_1.0 {
                self.locals[self.depth as usize].insert(s.clone(), 
                    a[j].clone()
                );
            }
            
            if let EconValue::Str(ref s) = &temp_2.0 {
                self.locals[self.depth as usize].insert(s.clone(), 
                    a[right].clone()
                );
            }
            
            let goto_point = self.current;
            let condition = self.val_expression()?;
            
            match condition {
                EconValue::Bool(b) => {
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
     
    fn quicksort(&mut self, a: &mut [EconValue], temp_1: &(EconValue, Option<EconValue>), temp_2: &(EconValue, Option<EconValue>)) -> Result<(), String> {
        if a.len() > 1 {
            let q = self.partition(a, temp_1, temp_2)?;
            self.quicksort(&mut a[..q], temp_1, temp_2)?;
            self.quicksort(&mut a[q+1..], temp_1, temp_2)?;
        }

        Ok(())
    }
    
    fn sort_impl(&mut self) -> Result<EconValue, String> {
        self.eat();
        self.consume(Token::LeftParen, "Expect '(' after Sort Function.".to_string())?;
        let right = self.val_expression()?;
        
        match right {
            EconValue::Arr(a) => {
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
                Ok(EconValue::Arr(new_vec))
            }
            _ => {
                self.error("Invalid literal/variable in Sort Function arguments.".to_string())
            }
        }
    }
    
    fn primary(&mut self) -> Result<EconValue, String> {
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
                    Function::ToString => {
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
            Token::Nil => {
                self.eat();
                Ok(EconValue::Nil)
            }
            Token::Num(n) => {
                self.eat();
                Ok(EconValue::Num(n))
            }
            Token::Bool(b) => {
                self.eat();
                Ok(EconValue::Bool(b))
            }
            Token::Str(s) => {
                self.eat();
                Ok(EconValue::Str(s))
            }
            Token::LeftCurl => {
                self.eat();
                self.locals.push(HashMap::new());
                self.constraints.push(HashMap::new());
                self.depth += 1;
                let obj = self.block()?;
                self.locals.pop();
                self.constraints.pop();
                self.depth -= 1;
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
                    Ok(EconValue::Nil)
                } else {
                    let value = if v.0 >= 0 {
                        match self.locals[(self.depth-v.0) as usize].get(&v.1) {
                            Some(EconValue::Num(n)) => {
                                Ok(EconValue::Num(*n))
                            }
                            Some(EconValue::Str(s)) => {
                                Ok(EconValue::Str(s.to_string()))
                            }
                            Some(EconValue::Bool(b)) => {
                                Ok(EconValue::Bool(*b))
                            }
                            Some(EconValue::Obj(o)) => {
                                Ok(EconValue::Obj(o.clone()))
                            }
                            Some(EconValue::Arr(a)) => {
                                Ok(EconValue::Arr(a.clone()))
                            }
                            _ => {
                                Ok(EconValue::Nil)
                            }
                        }
                    } else {
                        let mut res = None;
                        for i in (0..=self.depth).rev() {
                            res = match self.locals[i as usize].get(&v.1) {
                                Some(EconValue::Num(n)) => {
                                    Some(EconValue::Num(*n))
                                }
                                Some(EconValue::Str(s)) => {
                                    Some(EconValue::Str(s.to_string()))
                                }
                                Some(EconValue::Bool(b)) => {
                                    Some(EconValue::Bool(*b))
                                }
                                Some(EconValue::Obj(o)) => {
                                    Some(EconValue::Obj(o.clone()))
                                }
                                Some(EconValue::Arr(a)) => {
                                    Some(EconValue::Arr(a.clone()))
                                }
                                _ => {
                                    None
                                }
                            };
                            
                            if res.is_some() {
                                break;
                            }
                        }
                        if let Some(r) = res {
                            Ok(r)
                        } else {
                            Ok(EconValue::Nil)
                        }
                    };
                    
                    match value {
                        Ok(EconValue::Obj(o)) => {
                            let mut c = EconValue::Obj(o.clone());
                        
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
                                            EconValue::Str(s) => {
                                                match c.clone() {
                                                    EconValue::Obj(oo) => {
                                                        if let Some(v) = oo.data.get(&s) {
                                                            c = v.clone();
                                                        } else {
                                                            c = EconValue::Nil;
                                                        }
                                                    }
                                                    _ => {
                                                        return self.error("Expect key after selecter.".to_string())
                                                    }
                                                }
                                            }
                                            EconValue::Num(n) => {
                                                match c.clone() {
                                                    EconValue::Arr(aa) => {
                                                        if n < 0.0 {
                                                            c = EconValue::Nil
                                                        } else {
                                                            if let Some(v) = aa.get(n as usize) {
                                                                c = v.clone()
                                                            } else {
                                                                c = EconValue::Nil
                                                            }
                                                        }
                                                    }
                                                    EconValue::Str(ss) => {
                                                        if n < 0.0 {
                                                            c = EconValue::Nil
                                                        } else {
                                                            if let Some(v) = ss.chars().nth(n as usize) {
                                                                c = EconValue::Str(v.to_string())
                                                            } else {
                                                                c = EconValue::Nil
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
                        Ok(EconValue::Arr(a)) => {
                            let mut c = EconValue::Arr(a.clone());
                            
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
                                            EconValue::Str(s) => {
                                                match c.clone() {
                                                    EconValue::Obj(oo) => {
                                                        if let Some(v) = oo.data.get(&s) {
                                                            c = v.clone();
                                                        } else {
                                                            c = EconValue::Nil;
                                                        }
                                                    }
                                                    _ => {
                                                        return self.error("Expect index after selecter.".to_string())
                                                    }
                                                }
                                            }
                                            EconValue::Num(n) => {
                                                match c.clone() {
                                                    EconValue::Arr(aa) => {
                                                        if n < 0.0 {
                                                            c = EconValue::Nil
                                                        } else {
                                                            if let Some(v) = aa.get(n as usize) {
                                                                c = v.clone()
                                                            } else {
                                                                c = EconValue::Nil
                                                            }
                                                        }
                                                    }
                                                    EconValue::Str(ss) => {
                                                        if n < 0.0 {
                                                            c = EconValue::Nil
                                                        } else {
                                                            if let Some(v) = ss.chars().nth(n as usize) {
                                                                c = EconValue::Str(v.to_string())
                                                            } else {
                                                                c = EconValue::Nil
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
                        Ok(EconValue::Str(s)) => {
                            match self.peek() {
                                Token::Dot => {
                                    self.eat();
                                    let expr = self.val_expression()?;
                                    match expr {
                                        EconValue::Num(n) => {
                                            if n < 0.0 {
                                                Ok(EconValue::Nil)
                                            } else {
                                                if let Some(v) = s.chars().nth(n as usize) {
                                                    Ok(EconValue::Str(v.to_string()))
                                                } else {
                                                    Ok(EconValue::Nil)
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
                                        EconValue::Num(n) => {  
                                            self.consume(Token::RightBracket, "Expect ']' after String Variable.".to_string())?;
                                            if n < 0.0 {
                                                Ok(EconValue::Nil)
                                            } else {
                                                if let Some(v) = s.chars().nth(n as usize) {
                                                    Ok(EconValue::Str(v.to_string()))
                                                } else {
                                                    Ok(EconValue::Nil)
                                                }
                                            }
                                            
                                        }
                                        _ => {
                                            self.error("Expect index after '['".to_string())
                                        }
                                    }
                                }
                                _ => {
                                    Ok(EconValue::Str(s.clone()))
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
                Ok(EconValue::Nil) 
            }
        }    
    }
    
    fn val_expression(&mut self) -> Result<EconValue, String> {
        self.equality()
    }
    
    fn array_value(&mut self) -> Result<EconValue, String> {
        Ok(self.val_expression()?)
    }
    
    fn array(&mut self) -> Result<EconValue, String> {
        let mut result = vec!();
        
        while !self.check(Token::RightBracket) && !self.at_end() {
            let val = self.array_value()?;
            let checked_value = self.check_val_with_constraint(val)?;
            result.push(checked_value);
            if !self.check(Token::RightBracket) {
                self.consume(Token::Comma, "Expect ','.".to_string())?;  
            }
        }
        
        self.consume(Token::RightBracket, "Expect ']' after array.".to_string())?;
        Ok(EconValue::Arr(result))
    }
    
    fn key(&mut self) -> Result<(String, EconValue), String> {
        let v_key = self.val_expression()?;
            
        if let EconValue::Str(s) = v_key {
            self.consume(Token::Colon, "Expected ':' after Key identifier".to_string())?;
            Ok((s.clone(), self.val_expression()?))
        } else {
            self.error(format!("Expected Key got: {}.", v_key))
        }
    }

    fn constraint_pre_process(&mut self) -> Result<(), String> {
        loop {
            match self.peek().clone() {
                Token::ConstraintMacro => {
                    self.eat();
                    self.consume(Token::LeftCurl, "Expected '{' after '@'.".to_string())?;

                    match self.peek().clone() {
                        Token::Str(s) => {
                            self.eat();
                            self.consume(Token::Comma, "Expected ',' after Constraint Type.".to_string())?;
                            let start_of = self.current;
                            
                            if let Some(mut v) = self.constraints[self.depth as usize].get_mut(s.as_str()) {
                                v.push((start_of, false));
                            } else {
                                self.constraints[self.depth as usize].insert(s.clone(), vec!((start_of, false)));
                            }
                            
                            loop {
                                if self.match_single(Token::RightCurl) {
                                    break;
                                }
                                self.eat();
                            }

                            if self.check(Token::ConstraintMacro) || self.check(Token::ErrorMacro) {
                                continue;    
                            }
                        }
                        Token::Nil => {
                            self.eat();
                            self.consume(Token::Comma, "Expected ',' after Constraint Type.".to_string())?;
                            let start_of = self.current;
                            
                            if let Some(mut v) = self.constraints[self.depth as usize].get_mut("nil") {
                                v.push((start_of, false));
                            } else {
                                self.constraints[self.depth as usize].insert("nil".to_string(), vec!((start_of, false)));
                            }
                            
                            loop {
                                if self.match_single(Token::RightCurl) {
                                    break;
                                }
                                self.eat();
                            }

                            if self.check(Token::ConstraintMacro) || self.check(Token::ErrorMacro) {
                                continue;    
                            }
                        }
                        _ => { return self.error("Constraint Macro preprocessor Error.".to_string()); }
                    }
                }
                Token::ErrorMacro => {
                    self.eat();
                    self.consume(Token::LeftCurl, "Expected '{' after '@!'.".to_string())?;

                    match self.peek().clone() {
                        Token::Str(s) => {
                            self.eat();                        
                            self.consume(Token::Comma, "Expected ',' after Constraint Type.".to_string())?;
                            let start_of = self.current;
                            
                            if let Some(mut v) = self.constraints[self.depth as usize].get_mut(s.as_str()) {
                                v.push((start_of, true));
                            } else {
                                self.constraints[self.depth as usize].insert(s.clone(), vec!((start_of, true)));
                            }
                            
                            loop {
                                if self.match_single(Token::RightCurl) {
                                    break;
                                }
                                self.eat();
                            }

                            if self.check(Token::ConstraintMacro) || self.check(Token::ErrorMacro) {
                                continue;    
                            }
                        }
                        Token::Nil => {
                            self.eat();                        
                            self.consume(Token::Comma, "Expected ',' after Constraint Type.".to_string())?;
                            let start_of = self.current;
                            
                            if let Some(mut v) = self.constraints[self.depth as usize].get_mut("nil") {
                                v.push((start_of, true));
                            } else {
                                self.constraints[self.depth as usize].insert("nil".to_string(), vec!((start_of, true)));
                            }
                            
                            loop {
                                if self.match_single(Token::RightCurl) {
                                    break;
                                }
                                self.eat();
                            }

                            if self.check(Token::ConstraintMacro) || self.check(Token::ErrorMacro) {
                                continue;    
                            }
                        }
                        _ => { return self.error("Error Macro preprocessor Error.".to_string());  }
                    }
                   
                }
                _ => { break; }
            }
        }
        
        Ok(())
    }

    fn check_val_with_constraint(&mut self, input: EconValue) -> Result<EconValue, String> {
        match input {
            EconValue::Str(s) => {
                if !self.in_constraint {
                    let mut str_to_use : Option<EconValue> = None;

                    for depth in (0..=self.depth).rev() {
                        let constr_vec = if let Some(cv) = self.constraints[depth as usize].get("string") {
                            Some(cv.clone())
                        } else {
                            None
                        };
        
                        if let Some(cv) = constr_vec {
                            self.in_constraint = true;
                            for cst in cv.iter() {
                                let return_to = self.current;
                                self.current = cst.0;
        
                                let temp_var = self.create_temp_var()?;
                                if let EconValue::Str(ref tv) = &temp_var.0 {
                                    if let Some(ntu) = &str_to_use {
                                        self.locals[self.depth as usize].insert(tv.clone(), 
                                            ntu.clone()
                                        );
                                    } else {
                                        self.locals[self.depth as usize].insert(tv.clone(), 
                                            EconValue::Str(s.to_string())
                                        );
                                    }
                                }
                                
                                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                                let condition = self.val_expression()?;
                                self.consume(Token::Comma, "Expect ',' after condition.".to_string())?;
                                let val = self.val_expression()?;

                                
                                
                                match condition {
                                    EconValue::Bool(true) => {
                                        if cst.1 {
                                            match val {
                                                EconValue::Str(s) => {
                                                    return self.error(format!("{}", s));
                                                }
                                                _ => return self.error(format!("{}", val))
                                            }
                                            
                                        }
                                        str_to_use = Some(val);
                                    }
                                    EconValue::Bool(false) => { }
                                    _ => {
                                        return self.error("Constraint condition must be boolean.".to_string());
                                    }
                                }
    
                                self.restore_temp_var(temp_var);
                                self.current = return_to;
                            }
                            self.in_constraint = false;
                        }
                    }
                    
                    if let Some(value_to_use) = str_to_use {
                        Ok(value_to_use)
                    } else {
                        Ok(EconValue::Str(s))
                    }
                } else {
                    Ok(EconValue::Str(s))
                }
            }
            EconValue::Bool(b) => {
                if !self.in_constraint {
                    let mut bool_to_use : Option<EconValue> = None;

                    for depth in (0..=self.depth).rev() {
                        let constr_vec = if let Some(cv) = self.constraints[depth as usize].get("bool") {
                            Some(cv.clone())
                        } else {
                            None
                        };
        
                        if let Some(cv) = constr_vec {
                            self.in_constraint = true;
                            for cst in cv.iter() {
                                let return_to = self.current;
                                self.current = cst.0;
        
                                let temp_var = self.create_temp_var()?;
                                if let EconValue::Str(ref s) = &temp_var.0 {
                                    if let Some(ntu) = &bool_to_use {
                                        self.locals[self.depth as usize].insert(s.clone(), 
                                            ntu.clone()
                                        );
                                    } else {
                                        self.locals[self.depth as usize].insert(s.clone(), 
                                            EconValue::Bool(b)
                                        );
                                    }
                                }
                                
                                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                                let condition = self.val_expression()?;
                                self.consume(Token::Comma, "Expect ',' after condition.".to_string())?;
                                let val = self.val_expression()?;
                                
                                match condition {
                                    EconValue::Bool(true) => {
                                        if cst.1 {
                                            match val {
                                                EconValue::Str(s) => {
                                                    return self.error(format!("{}", s));
                                                }
                                                _ => return self.error(format!("{}", val))
                                            }
                                            
                                        }
                                        bool_to_use = Some(val);
                                    }
                                    EconValue::Bool(false) => { }
                                    _ => {
                                        return self.error("Constraint condition must be boolean.".to_string());
                                    }
                                }
    
                                self.restore_temp_var(temp_var);
                                self.current = return_to;
                            }
                            self.in_constraint = false;
                        }
                    }
                    
                    if let Some(value_to_use) = bool_to_use {
                        Ok(value_to_use)
                    } else {
                        Ok(EconValue::Bool(b))
                    }
                } else {
                    Ok(EconValue::Bool(b))
                }
            }
            EconValue::Num(n) => {
                if !self.in_constraint {
                    let mut num_to_use : Option<EconValue> = None;

                    for depth in (0..=self.depth).rev() {
                        let constr_vec = if let Some(cv) = self.constraints[depth as usize].get("number") {
                            Some(cv.clone())
                        } else {
                            None
                        };
        
                        if let Some(cv) = constr_vec {
                            self.in_constraint = true;
                            for cst in cv.iter() {
                                let return_to = self.current;
                                self.current = cst.0;
        
                                let temp_var = self.create_temp_var()?;
                                if let EconValue::Str(ref s) = &temp_var.0 {
                                    if let Some(ntu) = &num_to_use {
                                        self.locals[self.depth as usize].insert(s.clone(), 
                                            ntu.clone()
                                        );
                                    } else {
                                        self.locals[self.depth as usize].insert(s.clone(), 
                                            EconValue::Num(n)
                                        );
                                    }
                                }
                                
                                self.consume(Token::Arrow, "Expect '=>' after iterator.".to_string())?;
                                let condition = self.val_expression()?;
                                self.consume(Token::Comma, "Expect ',' after condition.".to_string())?;
                                let val = self.val_expression()?;
                                
                                match condition {
                                    EconValue::Bool(true) => {
                                        if cst.1 {
                                            match val {
                                                EconValue::Str(s) => {
                                                    return self.error(format!("{}", s));
                                                }
                                                _ => return self.error(format!("{}", val))
                                            }
                                            
                                        }
                                        num_to_use = Some(val);
                                    }
                                    EconValue::Bool(false) => { }
                                    _ => {
                                        return self.error("Constraint condition must be boolean.".to_string());
                                    }
                                }
    
                                self.restore_temp_var(temp_var);
                                self.current = return_to;
                            }
                            self.in_constraint = false;
                        }
                    }
                    
                    if let Some(value_to_use) = num_to_use {
                        Ok(value_to_use)
                    } else {
                        Ok(EconValue::Num(n))
                    }
                } else {
                    Ok(EconValue::Num(n))
                }
            }
            EconValue::Nil => {
                if !self.in_constraint {
                    let mut nil_to_use : Option<EconValue> = None;

                    for depth in (0..=self.depth).rev() {
                        let constr_vec = if let Some(cv) = self.constraints[depth as usize].get("nil") {
                            Some(cv.clone())
                        } else {
                            None
                        };
        
                        if let Some(cv) = constr_vec {
                            self.in_constraint = true;
                            for cst in cv.iter() {
                                let return_to = self.current;
                                self.current = cst.0;
        
                                let temp_var = self.create_temp_var()?;
                                if let EconValue::Str(ref s) = &temp_var.0 {
                                    if let Some(ntu) = &nil_to_use {
                                        self.locals[self.depth as usize].insert(s.clone(), 
                                            ntu.clone()
                                        );
                                    } else {
                                        self.locals[self.depth as usize].insert(s.clone(), 
                                            EconValue::Nil
                                        );
                                    }
                                }
                                
                                self.consume(Token::Arrow, "Expect '=>' after reference.".to_string())?;
                                let condition = self.val_expression()?;
                                self.consume(Token::Comma, "Expect ',' after condition.".to_string())?;
                                let val = self.val_expression()?;
                                
                                match condition {
                                    EconValue::Nil => {
                                        if cst.1 {
                                            match val {
                                                EconValue::Str(s) => {
                                                    return self.error(format!("{}", s));
                                                }
                                                _ => return self.error(format!("{}", val))
                                            }
                                            
                                        }
                                        nil_to_use = Some(val);
                                    }
                                    _ => {
                                        return self.error("Constraint condition must be boolean.".to_string());
                                    }
                                }
    
                                self.restore_temp_var(temp_var);
                                self.current = return_to;
                            }
                            self.in_constraint = false;
                        }
                    }
                    
                    if let Some(value_to_use) = nil_to_use {
                        Ok(value_to_use)
                    } else {
                        Ok(EconValue::Nil)
                    }
                } else {
                    Ok(EconValue::Nil)
                }
            }
            _ => { Ok(input) }
        }
    }
    
    fn expression(&mut self) -> Result<(String, EconValue), String> {
        self.key()
    }
    
    fn object(&mut self) -> Result<EconValue, String> {
        if self.match_single(Token::LeftCurl) {
            self.locals.push(HashMap::new());
            self.constraints.push(HashMap::new());
            self.depth += 1;
            self.block()
        } else {
            self.error("Object Error.".to_string())
        }
    }
    
    fn block(&mut self) -> Result<EconValue, String> {
        let mut result = EconObj::new();
        
        while !self.check(Token::RightCurl) && !self.at_end() {
            self.constraint_pre_process()?;

            let key_val = self.expression()?;
            if let Some(_) = result.data.get(&key_val.0) {
                return self.error("Duplicate Key.".to_string());
            } else {
                let checked_value = self.check_val_with_constraint(key_val.1)?;
                result.data.insert(key_val.0.clone(), checked_value.clone());
                self.locals[self.depth as usize].insert(key_val.0, checked_value);
            }
            if !self.check(Token::RightCurl) {
                self.consume(Token::Comma, "Expect ','.".to_string())?;  
            }
        }
        self.consume(Token::RightCurl, "Expect '}' after block.".to_string())?;
        Ok(EconValue::Obj(result))
    }

    pub fn parse(&mut self, lexer: &mut EconLexer, debug: bool) -> Result<EconValue, String> {
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
                }) => {
                    break;
                }
                Ok(TokenData {
                    token: Token::Macro(mac),
                    line: _,
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

        match self.object() {
            Ok(value) => {
                match value {
                    EconValue::Obj(v) => {
                        if debug { 
                            println!("[Completed in {} ms]", now.elapsed().as_millis()); 
                            println!("{}", &v);
                        }
                        
                        Ok(EconValue::Obj(v))
                    }
                    _ => {
                        self.error(String::from("Object not found."))
                    }
                }
            }
            Err(e) => Err(e)
        }
    }
}