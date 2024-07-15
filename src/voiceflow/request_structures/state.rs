use std::ops::{Deref, DerefMut};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct State{
    variables: Vec<Value>
}

impl Deref for State {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

impl DerefMut for State{
    fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.variables
    }
}
impl State{
    pub fn new(variables: Vec<Value>) -> Self{
        Self{
            variables
        }
    }
}
impl Default for State{
    fn default() -> Self {
        Self{
            variables: vec![]
        }
    }
}