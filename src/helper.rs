use serenity::{model::channel, Result};
use tracing::error;

pub trait SerenityErrorHandler {
    fn handle_result(&self);
}

impl SerenityErrorHandler for Result<channel::Message> {
    fn handle_result(&self) {
        if let Err(e) = self {
            error!("Error : {:?}", e);
        }
    }
}

impl SerenityErrorHandler for Result<()> {
    fn handle_result(&self) {
        if let Err(e) = self {
            error!("Error : {:?}", e);
        }
    }
}
