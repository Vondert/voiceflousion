use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonCallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_mark: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<bool>,
}

impl ButtonCallbackData {
    /// Creates a new `ButtonCallbackData` instance.
    ///
    /// # Parameters
    ///
    /// * `index` - An optional index value.
    /// * `timestamp_mark` - An optional timestamp mark.
    /// * `direction` - An optional direction (true for forward, false for backward).
    ///
    /// # Returns
    ///
    /// A new `ButtonCallbackData` instance.
    fn new(index: Option<usize>, timestamp_mark: Option<i64>, direction: Option<bool>) -> Self {
        Self {
            index,
            timestamp_mark,
            direction,
        }
    }

    /// Converts the `ButtonCallbackData` instance to a JSON string.
    ///
    /// # Returns
    ///
    /// The JSON string if serialization succeeds or an empty string if serialization fails.
    pub fn to_json_string(self) -> String{
        serde_json::to_string(&self).unwrap_or_else(|_| "".to_string())
    }

    /// Returns a reference to the index, if any.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Returns a reference to the timestamp mark, if any.
    pub fn timestamp_mark(&self) -> Option<i64> {
        self.timestamp_mark
    }

    /// Returns a reference to the direction, if any.
    pub fn direction(&self) -> Option<bool> {
        self.direction
    }
}

#[derive(Debug, Default)]
pub struct ButtonCallbackDataBuilder {
    index: Option<usize>,
    timestamp_mark: Option<i64>,
    direction: Option<bool>,
}

impl ButtonCallbackDataBuilder {
    /// Creates a new `ButtonCallbackDataBuilder`.
    ///
    /// # Returns
    ///
    /// A new `ButtonCallbackDataBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the index value.
    ///
    /// # Parameters
    ///
    /// * `index` - The index value to set.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Sets the timestamp mark.
    ///
    /// # Parameters
    ///
    /// * `timestamp_mark` - The timestamp mark to set.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    pub fn timestamp_mark(mut self, timestamp_mark: i64) -> Self {
        self.timestamp_mark = Some(timestamp_mark);
        self
    }

    /// Sets the direction.
    ///
    /// # Parameters
    ///
    /// * `direction` - The direction value to set.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    pub fn direction(mut self, direction: bool) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Builds the `ButtonCallbackData` instance.
    ///
    /// # Returns
    ///
    /// A new `ButtonCallbackData` instance.
    pub fn build(self) -> ButtonCallbackData {
        ButtonCallbackData::new(self.index, self.timestamp_mark, self.direction)
    }
}