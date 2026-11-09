mod lexer_regex;
mod lexer_manual;
mod rules;
mod ast;
mod parser;

use regex::Regex;
use std::fs;
use std::env;
use std::io::Write;
use rules::{RULES, Token};
use parser::Parser;

// Rules-based lexer using rules.rs
fn lex(mut input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    while !input.is_empty() {
        input = input.trim_start();
        if input.is_empty() { break; }
        let mut matched = false;
        for rule in RULES.iter() {
            if let Some(m) = rule.regex.find(input) {
                let lexeme = m.as_str();
                // Special check: invalid identifier like `2abc`
                if Regex::new(r"^\d+[a-zA-Z_]").unwrap().is_match(input) {
                    tokens.push(Token::Error(format!("Invalid identifier: {}", lexeme)));
                    input = &input[m.end()..];
                    matched = true;
                    break;
                }
                tokens.push((rule.token_type)(lexeme));
                input = &input[m.end()..];
                matched = true;
                break;
            }
        }
        if !matched {
            tokens.push(Token::Error(format!("Unexpected character: {}", &input[..1])));
            input = &input[1..];
        }
    }
    tokens
}

fn write_regex_tokens_to_file(tokens: &[lexer_regex::Token], filename: &str) {
    let mut file = fs::File::create(filename).expect("Failed to create file");
    for token in tokens {
        let token_str = match token {
            lexer_regex::Token::Function => "T_FUNCTION".to_string(),
            lexer_regex::Token::Int => "T_INT".to_string(),
            lexer_regex::Token::Float => "T_FLOAT".to_string(),
            lexer_regex::Token::String => "T_STRING".to_string(),
            lexer_regex::Token::Bool => "T_BOOL".to_string(),
            lexer_regex::Token::Identifier(s) => format!("T_IDENTIFIER(\"{}\")", s),
            lexer_regex::Token::IntLit(n) => format!("T_INTLIT({})", n),
            lexer_regex::Token::FloatLit(f) => format!("T_FLOATLIT({})", f),
            lexer_regex::Token::StringLit(s) => format!("T_STRINGLIT(\"{}\")", s),
            lexer_regex::Token::BoolLit(b) => format!("T_BOOLLIT({})", b),
            lexer_regex::Token::Return => "T_RETURN".to_string(),
            lexer_regex::Token::If => "T_IF".to_string(),
            lexer_regex::Token::Else => "T_ELSE".to_string(),
            lexer_regex::Token::While => "T_WHILE".to_string(),
            lexer_regex::Token::For => "T_FOR".to_string(),
            lexer_regex::Token::AssignOp => "T_ASSIGNOP".to_string(),
            lexer_regex::Token::EqualsOp => "T_EQUALSOP".to_string(),
            lexer_regex::Token::NotEqualsOp => "T_NOTEQUALSOP".to_string(),
            lexer_regex::Token::LessEqOp => "T_LESSEQOP".to_string(),
            lexer_regex::Token::GreaterEqOp => "T_GREATEREQOP".to_string(),
            lexer_regex::Token::LessOp => "T_LESSOP".to_string(),
            lexer_regex::Token::GreaterOp => "T_GREATEROP".to_string(),
            lexer_regex::Token::AndOp => "T_ANDOP".to_string(),
            lexer_regex::Token::OrOp => "T_OROP".to_string(),
            lexer_regex::Token::BitAndOp => "T_BITANDOP".to_string(),
            lexer_regex::Token::BitOrOp => "T_BITOROP".to_string(),
            lexer_regex::Token::ParenL => "T_PARENL".to_string(),
            lexer_regex::Token::ParenR => "T_PARENR".to_string(),
            lexer_regex::Token::BraceL => "T_BRACEL".to_string(),
            lexer_regex::Token::BraceR => "T_BRACER".to_string(),
            lexer_regex::Token::BracketL => "T_BRACKETL".to_string(),
            lexer_regex::Token::BracketR => "T_BRACKETR".to_string(),
            lexer_regex::Token::Comma => "T_COMMA".to_string(),
            lexer_regex::Token::Semicolon => "T_SEMICOLON".to_string(),
            lexer_regex::Token::Quotes => "T_QUOTES".to_string(),
            lexer_regex::Token::Comment(s) => format!("T_COMMENT(\"{}\")", s),
            lexer_regex::Token::Error(s) => format!("T_ERROR(\"{}\")", s),
        };
        writeln!(file, "{}", token_str).expect("Failed to write to file");
    }
}

fn write_manual_tokens_to_file(tokens: &[lexer_manual::Token], filename: &str) {
    let mut file = fs::File::create(filename).expect("Failed to create file");
    for token in tokens {
        let token_str = match token {
            // Keywords
            lexer_manual::Token::Function => "T_FUNCTION".to_string(),
            lexer_manual::Token::Int => "T_INT".to_string(),
            lexer_manual::Token::Float => "T_FLOAT".to_string(),
            lexer_manual::Token::String => "T_STRING".to_string(),
            lexer_manual::Token::Bool => "T_BOOL".to_string(),
            lexer_manual::Token::Char => "T_CHAR".to_string(),
            lexer_manual::Token::Void => "T_VOID".to_string(),
            lexer_manual::Token::Short => "T_SHORT".to_string(),
            lexer_manual::Token::Long => "T_LONG".to_string(),
            lexer_manual::Token::Signed => "T_SIGNED".to_string(),
            lexer_manual::Token::Unsigned => "T_UNSIGNED".to_string(),
            lexer_manual::Token::Struct => "T_STRUCT".to_string(),
            lexer_manual::Token::Enum => "T_ENUM".to_string(),
            lexer_manual::Token::Typedef => "T_TYPEDEF".to_string(),
            lexer_manual::Token::Return => "T_RETURN".to_string(),
            lexer_manual::Token::If => "T_IF".to_string(),
            lexer_manual::Token::Else => "T_ELSE".to_string(),
            lexer_manual::Token::While => "T_WHILE".to_string(),
            lexer_manual::Token::For => "T_FOR".to_string(),
            lexer_manual::Token::Do => "T_DO".to_string(),
            lexer_manual::Token::Switch => "T_SWITCH".to_string(),
            lexer_manual::Token::Case => "T_CASE".to_string(),
            lexer_manual::Token::Default => "T_DEFAULT".to_string(),
            lexer_manual::Token::Break => "T_BREAK".to_string(),
            lexer_manual::Token::Continue => "T_CONTINUE".to_string(),
            lexer_manual::Token::Goto => "T_GOTO".to_string(),
            
            // Literals
            lexer_manual::Token::Identifier(s) => format!("T_IDENTIFIER(\"{}\")", s),
            lexer_manual::Token::IntLit(n) => format!("T_INTLIT({})", n),
            lexer_manual::Token::FloatLit(f) => format!("T_FLOATLIT({})", f),
            lexer_manual::Token::StringLit(s) => format!("T_STRINGLIT(\"{}\")", s),
            lexer_manual::Token::BoolLit(b) => format!("T_BOOLLIT({})", b),
            lexer_manual::Token::CharLit(c) => format!("T_CHARLIT('{}')", c),
            
            // Operators
            lexer_manual::Token::AssignOp => "T_ASSIGNOP".to_string(),
            lexer_manual::Token::EqualsOp => "T_EQUALSOP".to_string(),
            lexer_manual::Token::NotEqualsOp => "T_NOTEQUALSOP".to_string(),
            lexer_manual::Token::LessEqOp => "T_LESSEQOP".to_string(),
            lexer_manual::Token::GreaterEqOp => "T_GREATEREQOP".to_string(),
            lexer_manual::Token::LessOp => "T_LESSOP".to_string(),
            lexer_manual::Token::GreaterOp => "T_GREATEROP".to_string(),
            lexer_manual::Token::AndOp => "T_ANDOP".to_string(),
            lexer_manual::Token::OrOp => "T_OROP".to_string(),
            lexer_manual::Token::BitAndOp => "T_BITANDOP".to_string(),
            lexer_manual::Token::BitOrOp => "T_BITOROP".to_string(),
            lexer_manual::Token::BitXorOp => "T_BITXOROP".to_string(),
            lexer_manual::Token::Plus => "T_PLUS".to_string(),
            lexer_manual::Token::Minus => "T_MINUS".to_string(),
            lexer_manual::Token::Star => "T_STAR".to_string(),
            lexer_manual::Token::Slash => "T_SLASH".to_string(),
            lexer_manual::Token::Percent => "T_PERCENT".to_string(),
            lexer_manual::Token::LeftShift => "T_LEFTSHIFT".to_string(),
            lexer_manual::Token::RightShift => "T_RIGHTSHIFT".to_string(),
            lexer_manual::Token::Not => "T_NOT".to_string(),
            lexer_manual::Token::BitwiseNot => "T_BITWISENOT".to_string(),
            
            // Assignment operators
            lexer_manual::Token::MultiplyAssign => "T_MULTIPLYASSIGN".to_string(),
            lexer_manual::Token::DivideAssign => "T_DIVIDEASSIGN".to_string(),
            lexer_manual::Token::ModuloAssign => "T_MODULOASSIGN".to_string(),
            lexer_manual::Token::AddAssign => "T_ADDASSIGN".to_string(),
            lexer_manual::Token::SubtractAssign => "T_SUBTRACTASSIGN".to_string(),
            lexer_manual::Token::LeftShiftAssign => "T_LEFTSHIFTASSIGN".to_string(),
            lexer_manual::Token::RightShiftAssign => "T_RIGHTSHIFTASSIGN".to_string(),
            lexer_manual::Token::BitwiseAndAssign => "T_BITWISEANDASSIGN".to_string(),
            lexer_manual::Token::BitwiseXorAssign => "T_BITWISEXORASSIGN".to_string(),
            lexer_manual::Token::BitwiseOrAssign => "T_BITWISEORASSIGN".to_string(),
            
            // Unary operators
            lexer_manual::Token::AddressOf => "T_ADDRESSOF".to_string(),
            lexer_manual::Token::Dereference => "T_DEREFERENCE".to_string(),
            lexer_manual::Token::PreIncrement => "T_PREINCREMENT".to_string(),
            lexer_manual::Token::PreDecrement => "T_PREDECREMENT".to_string(),
            lexer_manual::Token::PostIncrement => "T_POSTINCREMENT".to_string(),
            lexer_manual::Token::PostDecrement => "T_POSTDECREMENT".to_string(),
            
            // Delimiters
            lexer_manual::Token::ParenL => "T_PARENL".to_string(),
            lexer_manual::Token::ParenR => "T_PARENR".to_string(),
            lexer_manual::Token::BraceL => "T_BRACEL".to_string(),
            lexer_manual::Token::BraceR => "T_BRACER".to_string(),
            lexer_manual::Token::BracketL => "T_BRACKETL".to_string(),
            lexer_manual::Token::BracketR => "T_BRACKETR".to_string(),
            lexer_manual::Token::Comma => "T_COMMA".to_string(),
            lexer_manual::Token::Semicolon => "T_SEMICOLON".to_string(),
            lexer_manual::Token::Colon => "T_COLON".to_string(),
            lexer_manual::Token::Question => "T_QUESTION".to_string(),
            lexer_manual::Token::Dot => "T_DOT".to_string(),
            lexer_manual::Token::Arrow => "T_ARROW".to_string(),
            lexer_manual::Token::Quotes => "T_QUOTES".to_string(),
            
            // Comments
            lexer_manual::Token::Comment(s) => format!("T_COMMENT(\"{}\")", s),
            
            // Error
            lexer_manual::Token::Error(s) => format!("T_ERROR(\"{}\")", s),
        };
        writeln!(file, "{}", token_str).expect("Failed to write to file");
    }
}

fn write_ast_to_json(ast: &ast::Statement, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = ast_to_json(ast);
    fs::write(filename, json)?;
    Ok(())
}

fn ast_to_json(ast: &ast::Statement) -> String {
    match ast {
        ast::Statement::CompoundStatement { items } => {
            let items_json: Vec<String> = items.iter().map(|item| {
                match item {
                    ast::CompoundItem::Declaration(decl) => {
                        match decl {
                            ast::Declaration::VariableDeclaration { specifiers, declarators } => {
                                let spec_strs: Vec<String> = specifiers.iter().map(|s| format!("\"{}\"", type_specifier_to_string(s))).collect();
                                let decl_strs: Vec<String> = declarators.iter().map(|d| format!("\"{}\"", declarator_to_string(d))).collect();
                                format!(
                                    "{{\n        \"Declaration\": {{\n          \"specifiers\": [{}],\n          \"declarators\": [{}]\n        }}\n      }}",
                                    spec_strs.join(", "),
                                    decl_strs.join(", ")
                                )
                            }
                            ast::Declaration::FunctionDeclaration { specifiers, declarator } => {
                                let spec_strs: Vec<String> = specifiers.iter().map(|s| format!("\"{}\"", type_specifier_to_string(s))).collect();
                                format!(
                                    "{{\n        \"FunctionDeclaration\": {{\n          \"specifiers\": [{}],\n          \"declarator\": \"{}\"\n        }}\n      }}",
                                    spec_strs.join(", "),
                                    declarator_to_string(declarator)
                                )
                            }
                        }
                    }
                    ast::CompoundItem::Statement(stmt) => {
                        format!("{{\n        \"Statement\": {}\n      }}", ast_to_json(stmt))
                    }
                }
            }).collect();
            
            format!(
                "{{\n  \"CompoundStatement\": {{\n    \"items\": [\n{}\n    ]\n  }}\n}}",
                items_json.join(",\n")
            )
        }
        ast::Statement::ExpressionStatement { expression } => {
            let expr_json = match expression {
                Some(expr) => format!("{{\n        \"Some\": {}\n      }}", expression_to_json(expr)),
                None => "null".to_string(),
            };
            format!(
                "{{\n  \"ExpressionStatement\": {{\n    \"expression\": {}\n  }}\n}}",
                expr_json
            )
        }
        ast::Statement::IfStatement { condition, then_statement, else_statement } => {
            let else_json = match else_statement {
                Some(stmt) => format!("{{\n        \"Some\": {}\n      }}", ast_to_json(stmt)),
                None => "null".to_string(),
            };
            format!(
                "{{\n  \"IfStatement\": {{\n    \"condition\": {},\n    \"then_statement\": {},\n    \"else_statement\": {}\n  }}\n}}",
                expression_to_json(condition),
                ast_to_json(then_statement),
                else_json
            )
        }
        ast::Statement::WhileStatement { condition, statement } => {
            format!(
                "{{\n  \"WhileStatement\": {{\n    \"condition\": {},\n    \"statement\": {}\n  }}\n}}",
                expression_to_json(condition),
                ast_to_json(statement)
            )
        }
        ast::Statement::ForStatement { init, condition, increment, statement } => {
            let init_json = match init {
                Some(stmt) => format!("{{\n        \"Some\": {}\n      }}", ast_to_json(stmt)),
                None => "null".to_string(),
            };
            let cond_json = match condition {
                Some(expr) => format!("{{\n        \"Some\": {}\n      }}", expression_to_json(expr)),
                None => "null".to_string(),
            };
            let incr_json = match increment {
                Some(expr) => format!("{{\n        \"Some\": {}\n      }}", expression_to_json(expr)),
                None => "null".to_string(),
            };
            format!(
                "{{\n  \"ForStatement\": {{\n    \"init\": {},\n    \"condition\": {},\n    \"increment\": {},\n    \"statement\": {}\n  }}\n}}",
                init_json,
                cond_json,
                incr_json,
                ast_to_json(statement)
            )
        }
        ast::Statement::ReturnStatement { expression } => {
            let expr_json = match expression {
                Some(expr) => format!("{{\n        \"Some\": {}\n      }}", expression_to_json(expr)),
                None => "null".to_string(),
            };
            format!(
                "{{\n  \"ReturnStatement\": {{\n    \"expression\": {}\n  }}\n}}",
                expr_json
            )
        }
        _ => format!("\"{:?}\"", ast),
    }
}

fn expression_to_json(expr: &ast::Expression) -> String {
    match expr {
        ast::Expression::Identifier(name) => {
            format!("{{\n        \"Identifier\": \"{}\"\n      }}", name)
        }
        ast::Expression::Constant(constant) => {
            match constant {
                ast::Constant::Integer(val) => {
                    format!("{{\n        \"Constant\": {{\n          \"Integer\": {}\n        }}\n      }}", val)
                }
                ast::Constant::Float(val) => {
                    format!("{{\n        \"Constant\": {{\n          \"Float\": {}\n        }}\n      }}", val)
                }
                ast::Constant::Boolean(val) => {
                    format!("{{\n        \"Constant\": {{\n          \"Boolean\": {}\n        }}\n      }}", val)
                }
                _ => format!("\"{:?}\"", constant),
            }
        }
        ast::Expression::Assignment { left, operator, right } => {
            format!(
                "{{\n        \"Assignment\": {{\n          \"left\": {},\n          \"operator\": \"{}\",\n          \"right\": {}\n        }}\n      }}",
                expression_to_json(left),
                assignment_operator_to_string(operator),
                expression_to_json(right)
            )
        }
        ast::Expression::Binary { left, operator, right } => {
            format!(
                "{{\n        \"Binary\": {{\n          \"left\": {},\n          \"operator\": \"{}\",\n          \"right\": {}\n        }}\n      }}",
                expression_to_json(left),
                binary_operator_to_string(operator),
                expression_to_json(right)
            )
        }
        ast::Expression::Parenthesized(expr) => {
            format!(
                "{{\n        \"Parenthesized\": {}\n      }}",
                expression_to_json(expr)
            )
        }
        _ => format!("\"{:?}\"", expr),
    }
}

fn type_specifier_to_string(spec: &ast::TypeSpecifier) -> &str {
    match spec {
        ast::TypeSpecifier::Int => "Int",
        ast::TypeSpecifier::Float => "Float",
        ast::TypeSpecifier::Char => "Char",
        ast::TypeSpecifier::Void => "Void",
        _ => "Unknown",
    }
}

fn declarator_to_string(decl: &ast::Declarator) -> String {
    match decl {
        ast::Declarator::Identifier(name) => name.clone(),
        _ => "Unknown".to_string(),
    }
}

fn assignment_operator_to_string(op: &ast::AssignmentOperator) -> &str {
    match op {
        ast::AssignmentOperator::Assign => "Assign",
        ast::AssignmentOperator::AddAssign => "AddAssign",
        ast::AssignmentOperator::SubtractAssign => "SubtractAssign",
        ast::AssignmentOperator::MultiplyAssign => "MultiplyAssign",
        ast::AssignmentOperator::DivideAssign => "DivideAssign",
        _ => "Unknown",
    }
}

fn binary_operator_to_string(op: &ast::BinaryOperator) -> &str {
    match op {
        ast::BinaryOperator::Add => "Add",
        ast::BinaryOperator::Subtract => "Subtract",
        ast::BinaryOperator::Multiply => "Multiply",
        ast::BinaryOperator::Divide => "Divide",
        ast::BinaryOperator::Equal => "Equal",
        ast::BinaryOperator::NotEqual => "NotEqual",
        ast::BinaryOperator::Less => "Less",
        ast::BinaryOperator::Greater => "Greater",
        ast::BinaryOperator::LessEqual => "LessEqual",
        ast::BinaryOperator::GreaterEqual => "GreaterEqual",
        ast::BinaryOperator::LogicalAnd => "LogicalAnd",
        ast::BinaryOperator::LogicalOr => "LogicalOr",
        _ => "Unknown",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <source_file>", args[0]);
        return;
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read file");

    // Run regex lexer
    println!("--- Tokens (Regex Lexer) ---");
    let tokens_regex = lexer_regex::lex_with_regex(&code);
    for t in &tokens_regex {
        println!("{:?}", t);
    }

    // Run manual lexer
    println!("\n--- Tokens (Manual Lexer) ---");
    let tokens_manual = lexer_manual::lex_manual(&code);
    for t in &tokens_manual {
        println!("{:?}", t);
    }

    // Run rules-based lexer
    println!("\n--- Tokens (Rules-based Lexer) ---");
    let tokens_rules = lex(&code);
    for t in &tokens_rules {
        println!("T_{:?}", t);
    }

    // Test parser with manual lexer tokens
    println!("\n--- Parser Test ---");
    let mut parser = Parser::new(tokens_manual.clone());
    match parser.parse_statement() {
        Ok(ast) => {
            println!("AST generated successfully:");
            println!("{:#?}", ast);
            
            // Save AST to JSON file
            match write_ast_to_json(&ast, "ast_output.json") {
                Ok(_) => println!("\n✅ AST saved to ast_output.json"),
                Err(e) => println!("\n❌ Error saving AST to JSON: {}", e),
            }
        }
        Err(error) => {
            println!("Parser error: {:?}", error);
        }
    }

    // Write tokens to files
    write_regex_tokens_to_file(&tokens_regex, "regex_tokens.txt");
    write_manual_tokens_to_file(&tokens_manual, "manual_tokens.txt");

    println!("\nFiles have been written to:");
    println!("- regex_tokens.txt (Regex-based lexer)");
    println!("- manual_tokens.txt (Manual lexer)");
    println!("- ast_output.json (AST in JSON format)");
}
