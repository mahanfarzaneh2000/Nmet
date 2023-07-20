use std::{error::Error, process::exit};
use std::fs;
use std::env::args;

mod lexer;
mod ast;
use lexer::{Lexer, TokenType, Token};

// --- Static Compiler Defenition
static VERSION : &'static str = "v0.0.1-Beta";
static COPYRIGHT : &'static str = "Mahan Farzaneh 2023-2024";
static DEBUG : bool = true;

// -4 -> 4 neg
// 4 + 2 -> 4 2 +
// 4 * 3 + 6 -> 4 3 * 6 +
// 4 + (3 + 6) -> 3 6 + 4 +
// -(4 * cos(0) + 2 - 6) -> 4 cos(0) * 2 + 6 - neg

#[derive(Debug,PartialEq,Clone)]
enum Op {
    Plus,
    Sub,
    Multi,
    Devide,
}
impl Op {
    pub fn from_token_type(token: &Token) -> Self {
        match token.t_type {
            TokenType::Plus => return Self::Plus,
            TokenType::Minus => return Self::Sub,
            TokenType::Multi => return Self::Multi,
            TokenType::Devide => return Self::Devide,
            _ => {
                println!("Error: Unexpected Op Token ({}) at {}",token.literal,token.get_loc_string());
                exit(-1);
            }
        }
    }
}


#[derive(Debug,PartialEq,Clone)]
struct UnaryExpr {
    op: Op,
    right: Box<Expr>
}

#[derive(Debug,PartialEq,Clone)]
struct BinaryExpr {
    left: Box<Expr>,
    op: Op,
    right: Box<Expr>
}

#[derive(Debug,PartialEq,Clone)]
struct FunctionCall {
    identifier: String,
    args: Vec<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
struct ArrayIndex {
    identifier: String,
    indexer: Box<Expr>,
}

#[derive(Debug,PartialEq,Clone)]
enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Int(i32),
    Variable(String),
    FunctionCall(FunctionCall),
    ArrayIndex(ArrayIndex),
}
impl Expr {
    pub fn parse(lexer: &mut Lexer, left: Option<Expr>) -> Option<Self>{
        let token = lexer.expect_some_token();
        if token.t_type == TokenType::SemiColon {
            return None;
        }
        match token.t_type {
            TokenType::Plus | TokenType::Minus => {
                let op = Op::from_token_type(&token);
                if left.is_none() {
                    let single_token = lexer.expect_some_token();
                    if let TokenType::Int(val) = single_token.t_type {
                        let new_left = Self::Unary(UnaryExpr{op, right: Box::new(Expr::Int(val))});
                        return Self::parse(lexer,Some(new_left));
                    }
                    let Some(right) = Self::parse(lexer, None) else {
                        println!("Error: Operation Missing right side {}",token.get_loc_string());
                        exit(-1);
                    };
                    return Some(Self::Unary(UnaryExpr{op, right: Box::new(right)}));
                } else {
                    let Some(right) = Self::parse(lexer, None) else {
                        println!("Error: Operation Missing right side {}",token.get_loc_string());
                        exit(-1);
                    };
                    return Some(Self::Binary(BinaryExpr{left: Box::new(left.unwrap()),op, right: Box::new(right)}));
                }
            },
            TokenType::Multi | TokenType::Devide => {
                let op = Op::from_token_type(&token);
                let Some(right) = Self::parse(lexer, None) else {
                    println!("Error: Operation Missing right side {}",token.get_loc_string());
                    exit(-1);
                };
                if left.is_none() {
                    println!("Error: Operation Missing left side {}",token.get_loc_string());
                    exit(-1);
                } else {
                    return Some(Self::Binary(BinaryExpr{left: Box::new(left.unwrap()),op, right: Box::new(right)}));
                }
            },
            TokenType::Int(val) => {
                if left.is_some() {
                    println!("Error: Unexpected token without Op ({}) at {}",token.literal,token.get_loc_string());
                    exit(-1);
                }else{
                    let left = Some(Self::Int(val));
                    let right = Self::parse(lexer, left.clone());
                    if right.is_none() {
                        return left;
                    }else {
                        return right;
                    }
                }
            },
            _ => {
                println!("Error: Unexpected token ({}) at {}",token.literal,token.get_loc_string());
                exit(-1);
            }
        }
    } 
}


fn padding_right(str : &str) -> String {
    let mut text = String::with_capacity(20);
    text.push_str(str);
    for _ in 0..(20-str.len()) {
       text.push(' '); 
    }
    text
}

fn help_command() -> Result<(),Box<dyn Error>> {
    println!("Nemet {VERSION}");
    println!("Nemet Programmin Language Copyright: {COPYRIGHT}");
    println!("Project distributed Under MIT License");
    if DEBUG {
        println!("--- DEBUG MODE ---");
    }
    println!("\nnemet [Command] <path> (Options)");
    println!("Commands:");
    println!("\t{} Show help",padding_right("help"));
    println!("Options:");
    println!("\t{} Show help",padding_right("--help"));
    println!("\t{} Show Version",padding_right("--version"));
    Ok(())
}

fn compile_command(path: String) -> Result<(),Box<dyn Error>> {
    let source = fs::read_to_string(path.clone())
        .expect("Can not Read the file");
    let mut lexer = Lexer::new(path.clone(),source);
    let mut token = lexer.next_token();
    while !token.is_none() {
        println!("{:?}",token.unwrap());
        token = lexer.next_token();
    }
    Ok(())
}

fn main() -> Result<(),Box<dyn Error>> {
    let source = "-1 + 2 + 3;".to_string();
    let mut lexer = Lexer::new(String::new(),source);
    let expr = Expr::parse(&mut lexer,None);
    println!("{:#?}",expr.unwrap());
    return Ok(());
    let mut arg = args().into_iter();
    arg.next();
    loop {
        let Some(command) = arg.next() else {
            break;
        };
        match command.as_str() {
            "help" => {
                help_command()?;
                return Ok(());
            },
            "--help" => {
                help_command()?;
                return Ok(());
            },
            "--version" => {
                println!("{VERSION}");
                return Ok(());
            },
            _ => {
                compile_command(command.clone())?;
                return Ok(());
            },
        }
    }
    Ok(())
}
