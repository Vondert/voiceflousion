use std::ops::Deref;
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use crate::integrations::utils::SenderHttpClient;
use crate::voiceflow::{VoiceflousionError, VoiceflowBlock, VoiceflowMessage};
use crate::voiceflow::dialog_blocks::{VoiceflowButtons, VoiceflowCard, VoiceflowCarousel, VoiceflowImage, VoiceflowText};

#[async_trait]
pub trait Sender: Send + Sync{
    fn sender_http_client(&self) -> &SenderHttpClient;
    fn api_key(&self) -> &String;
    async fn send_message(&self, chat_id: &String, message: &VoiceflowMessage) -> Result<Vec<Response>, VoiceflousionError>{
        let sender_http_client = self.sender_http_client();
        let api_key = self.api_key();
        let mut responses = Vec::with_capacity(message.len());

        for block in message.deref() {
            match block {
                VoiceflowBlock::Text(text) => {
                    let result = self.send_text(text, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Image(image) =>{
                    let result = self.send_image(image, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                }
                VoiceflowBlock::Buttons(buttons) => {
                    let result = self.send_buttons(buttons, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Card(card) => {
                    let result = self.send_card(card, chat_id, sender_http_client, api_key).await?;
                    responses.push(result)
                },
                VoiceflowBlock::Carousel(carousel) => {
                    if !carousel.is_empty(){
                        let result = self.send_carousel(carousel, chat_id, sender_http_client, api_key).await?;
                        responses.push(result)
                    }
                },
            }
        }

        Ok(responses)
    }
    async fn send_text(&self, text: &VoiceflowText, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError>;
    async fn send_image(&self, image: &VoiceflowImage, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError>;
    async fn send_buttons(&self, buttons: &VoiceflowButtons, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError>;
    async fn send_card(&self, card: &VoiceflowCard, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError>;
    async fn send_carousel(&self, carousel: &VoiceflowCarousel, chat_id: &String, sender_http_client: &SenderHttpClient, api_key: &String) -> Result<Response, VoiceflousionError>;
}