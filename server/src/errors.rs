#![allow(clippy::match_single_binding)]

use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum ServerError {
    
}

impl Display for ServerError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            _ => unimplemented!()
        }
    }
}

impl Context for ServerError {}

#[derive(Debug)]
pub struct UnRecoverableError;

impl Display for UnRecoverableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A non-recoverable error has occurred.")
    }
}

impl Context for UnRecoverableError {}