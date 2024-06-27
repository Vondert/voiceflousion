use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct State{
    variables: Vec<Value>
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

