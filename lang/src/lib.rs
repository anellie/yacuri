#![feature(box_syntax)]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use crate::{
    error::Error,
    parser::{Module, Parser},
};
use alloc::vec::Vec;

mod compiler;
mod error;
mod lexer;
mod parser;
mod smol_str;
mod vm;

pub fn execute_program(program: &str) -> Result<Module, Vec<Error>> {
    let mut parser = Parser::new(program);
    parser.parse()
}
