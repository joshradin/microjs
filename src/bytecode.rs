//! The bytecode instruction set.

use crate::engine::Engine;

pub trait Instruction {
    fn bytes(&self) -> usize;
    fn execute(&self);
}