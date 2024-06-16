use std::io::BufRead;

use anyhow::{Error, Result};
pub mod model;

use model::{Clause, Literal};

pub struct Parser<'a> {
    reader: &'a mut dyn BufRead,
}

fn trim_new_line(s: &mut String) {
    if s.ends_with("\n") {
        s.pop();
        if s.ends_with("\r") {
            s.pop();
        }
    }
}

///Relevant ascii chars in a dimacs file
/// std::ascii:Char is experimental at the time this is written
#[repr(u8)]
enum Token {
    Space = 32,
    Minus = 45,
    Digit0 = 48,
    Digit1 = 49,
    Digit2 = 50,
    Digit3 = 51,
    Digit4 = 52,
    Digit5 = 53,
    Digit6 = 54,
    Digit7 = 55,
    Digit8 = 56,
    Digit9 = 57,
    SmallC = 99,
    SmallP = 112,
}
impl Token {
    /// Gets this ASCII character as a byte.
    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Gets this ASCII character as a `char` Unicode Scalar Value.
    #[inline]
    pub const fn to_char(self) -> char {
        self as u8 as char
    }
}

impl Into<u8> for Token {
    fn into(self) -> u8 {
        self.to_u8()
    }
}

impl TryFrom<u8> for Token {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            32 => Ok(Token::Space),
            45 => Ok(Token::Minus),
            48 => Ok(Token::Digit0),
            49 => Ok(Token::Digit1),
            50 => Ok(Token::Digit2),
            51 => Ok(Token::Digit3),
            52 => Ok(Token::Digit4),
            53 => Ok(Token::Digit5),
            54 => Ok(Token::Digit6),
            55 => Ok(Token::Digit7),
            56 => Ok(Token::Digit8),
            57 => Ok(Token::Digit9),
            99 => Ok(Token::SmallC),
            112 => Ok(Token::SmallP),
            _ => Err(Error::msg("Invalid cnf token")),
        }
    }
}

impl<'a> Parser<'a> {
    ///Create a new dimacs parser
    pub fn new(reader: &'a mut dyn BufRead) -> Self {
        Self { reader }
    }
    pub fn parse_into(&mut self, m: &mut model::Model) -> Result<()> {
        let mut line = String::new();
        let mut bytes_read = self.reader.read_line(&mut line)?;
        while bytes_read > 0 {
            trim_new_line(&mut line);
            //dimacs is ascii
            let line_bytes = line.as_bytes();
            let first_char = line_bytes[0];
            //lines starting with a 'c' are comments
            if first_char == Token::SmallC.into() {
                let cline = line.strip_prefix(Token::SmallC.to_char()).unwrap();
                let comment = cline.strip_prefix(' ').unwrap_or(cline);
                m.add_comment(comment.to_string());
            //lines starting with a 'p' are problem statements
            } else if first_char == Token::SmallP.into() {
                if !(line.starts_with("p cnf ")) {
                    return Err(Error::msg("not a dimacs cnf problem"));
                }
                //yes, reading the number of clauses and variables is possible
                // but not required
            } else {
                //Clause line
                let mut variable_name = String::new();
                let mut literal = Literal::new();
                let mut clause = Clause::new();
                for character in line_bytes.iter() {
                    let t = Token::try_from(*character)?;
                    match t {
                        //Can not happen as it's already checked
                        Token::SmallC | Token::SmallP => return Err(Error::msg("Invalid clause")),
                        Token::Minus => literal.negate(),
                        Token::Space => {
                            //End of the Literal
                            literal.var = variable_name.parse()?;
                            variable_name.clear();
                            m.add_literal(literal);
                            clause.push(literal);
                            literal = Literal::new();
                        }
                        Token::Digit0 => {
                            //End of the clause
                            m.add_clause(clause);
                            clause = Clause::new()
                        }
                        //It's a digit from a Variable
                        _ => variable_name.push(t.to_char()),
                    }
                }
            }
            line.clear();
            bytes_read = self.reader.read_line(&mut line)?;
            //TODO: if the last character is not a '0', the next line is part of the clause too
            //line.push(' ')
        }
        Ok(())
    }
}
