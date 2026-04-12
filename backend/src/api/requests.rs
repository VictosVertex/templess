use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub class_id: u16,
}
