pub enum InteractionType{
    Text(String),
    Button(String, String),
    Undefined(String)
}
impl InteractionType{
        pub fn new(message: String, button_path: Option<String>) -> Self{
        match button_path{
            Some(path) => InteractionType::Button(message, path),
            None => InteractionType::Text(message)
        }
    }
}