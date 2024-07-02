pub enum InteractionType{
    Text(String),
    Button(String),
    Undefined(String)
}
impl InteractionType{
    pub fn new(message: String, interaction_type: String) -> Self{
        match interaction_type.as_str(){
            "button" => InteractionType::Button(message),
            "text" => InteractionType::Text(message),
            _ => InteractionType::Undefined(message)
        }
    }
}