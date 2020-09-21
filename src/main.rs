use std::env;

use std::ops::Deref;
use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                if data == "ok" {
                    if let Some(ref reply_to_message) = message.reply_to_message {
                        if let MessageOrChannelPost::Message(ref reply_to_message) = reply_to_message.deref() {
                            api.send(DeleteMessage::new(&reply_to_message.chat, reply_to_message.id)).await?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
