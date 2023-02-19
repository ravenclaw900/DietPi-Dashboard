// Code generated by jtd-codegen for Rust v0.2.1

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "requestType")]
pub enum FrontendRequest {
    #[serde(rename = "COMMAND")]
    Command(FrontendRequestCommand),

    #[serde(rename = "PAGE")]
    Page(FrontendRequestPage),

    #[serde(rename = "TOKEN")]
    Token(FrontendRequestToken),
}

#[derive(Serialize, Deserialize)]
pub struct FrontendRequestCommand {
    #[serde(rename = "cmd")]
    pub cmd: String,

    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Box<Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FrontendRequestPage {
    #[serde(rename = "page")]
    pub page: String,
}

#[derive(Serialize, Deserialize)]
pub struct FrontendRequestToken {
    #[serde(rename = "token")]
    pub token: String,
}
