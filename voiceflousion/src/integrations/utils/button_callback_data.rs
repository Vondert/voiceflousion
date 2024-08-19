use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonCallbackData{
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_mark: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<bool>
}

impl ButtonCallbackData{
    fn new(builder: ButtonCallbackDataBuilder) -> Self{
        Self {
            index: builder.index,
            timestamp_mark: builder.timestamp_mark,
            direction: builder.direction,
        }
    }
    pub fn to_json_string(self) -> String{
        serde_json::to_string(&self).unwrap_or_else(|_| "".to_string())
    }

    pub fn index(&self) -> Option<usize>{
        self.index
    }

    pub fn timestamp_mark(&self) -> Option<i64>{
        self.timestamp_mark
    }

    pub fn direction(&self) -> Option<bool>{
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn index(mut self, index: usize) -> Self{
        self.index = Some(index);
        self
    }

    pub fn timestamp_mark(mut self, timestamp_mark: i64) -> Self{
        self.timestamp_mark = Some(timestamp_mark);
        self
    }

    pub fn direction(mut self, direction: bool) -> Self{
        self.direction = Some(direction);
        self
    }

    pub fn build(self) -> ButtonCallbackData{
        ButtonCallbackData::new(self)
    }
}