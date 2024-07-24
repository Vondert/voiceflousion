use std::ops::{Deref, DerefMut};
use serde::Serialize;
use serde_json::Value;

/// Represents the state for the Voiceflow API.
///
/// `State` contains a list of variables represented as JSON values.
#[derive(Debug, Serialize, Clone)]
pub struct State {
    /// The list of variables in the state.
    variables: Vec<Value>,
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
    /// Creates a new `State` with the specified variables.
    ///
    /// # Parameters
    ///
    /// * `variables` - A list of variables represented as JSON values.
    ///
    /// # Returns
    ///
    /// A new instance of `State`.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use voiceflousion::core::voiceflow::State;
    ///
    /// let state = State::new(vec![Value::String("example".to_string())]);
    /// ```
    pub fn new(variables: Vec<Value>) -> Self{
        Self{
            variables
        }
    }
}
impl Default for State{
    /// Creates a new `State` with an empty list of variables.
    ///
    /// # Returns
    ///
    /// A new instance of `State`.
    ///
    /// # Example
    ///
    /// ```
    /// use voiceflousion::core::voiceflow::State;
    ///
    /// let state = State::default();
    /// ```
    fn default() -> Self {
        Self{
            variables: vec![]
        }
    }
}