use core::fmt;
use std::collections::{BTreeSet, HashSet};

pub type Var = i64;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Sign {
    Positive,
    Negative,
}

///A Literal is a signed Variable
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Literal {
    pub var: Var,
    pub sign: Sign,
}
impl Literal {
    pub fn new() -> Self {
        Self {
            var: 0,
            sign: Sign::Positive,
        }
    }
    pub fn negate(&mut self) {
        self.sign = if self.sign == Sign::Positive {
            Sign::Negative
        } else {
            Sign::Negative
        };
    }
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.sign == Sign::Negative { "-" } else { "" },
            self.var
        )
    }
}

///Clause is an OR of Literals
#[derive(Debug)]
pub struct Clause(Vec<Literal>);

impl Clause {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, value: Literal) {
        self.0.push(value);
    }
}
impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for literal in self.0.iter() {
            write!(f, "{} ", literal)?;
        }
        write!(f, "0")
    }
}

///CNF representation of a boolean formula as an AND of Clauses
/// The comments of the parsed dimacs file are present and
/// the variables and Literals are kept seperately
#[derive(Debug)]
pub struct Model {
    comments: Vec<String>,
    variables: BTreeSet<Var>,
    literals: HashSet<Literal>,
    clauses: Vec<Clause>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            comments: Vec::new(),
            variables: BTreeSet::new(),
            literals: HashSet::new(),
            clauses: Vec::new(),
        }
    }
    pub fn add_comment(&mut self, value: String) {
        //let s = value.to_string();
        self.comments.push(value);
    }
    pub fn add_literal(&mut self, value: Literal) {
        self.variables.insert(value.var);
        self.literals.insert(value);
    }
    pub fn add_clause(&mut self, value: Clause) {
        self.clauses.push(value);
    }
}
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for comment in self.comments.iter() {
            writeln!(f, "c {}", comment)?;
        }
        writeln!(f, "p cnf {} {}", self.variables.len(), self.clauses.len())?;
        for clause in self.clauses.iter() {
            writeln!(f, "{}", clause)?;
        }
        Ok(())
    }
}
