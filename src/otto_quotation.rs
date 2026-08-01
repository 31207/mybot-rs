use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use nonebot_rs::message::UniMessage;
use nonebot_rs::message::FileType;
use builtin_plugins::matcher::prelude::*;
use nonebot_rs::event::MessageEvent;
use tokio::sync::Mutex;
use tracing::{Level, event};
pub struct OttoQuotation {
    path: String,
    file_paths: Mutex<Vec<String>>,
}

#[async_trait]
impl Handler<MessageEvent> for OttoQuotation {
    on_command!(MessageEvent, "来点电棍");

    async fn handle(&self, _: MessageEvent, matcher: Matcher<MessageEvent>) {
        let file_paths = self.file_paths.lock().await;
        if file_paths.is_empty() {
            matcher.send_text("没有可用的电棍语录").await;
            return;
        }
        let random_index: usize = (rand::random::<u32>() % file_paths.len() as u32) as usize;
        let selected_file = &file_paths[random_index];
        let audio_base64 = match std::fs::read(selected_file) {
            Ok(data) => BASE64_STANDARD.encode(data),
            Err(_) => {
                event!(Level::ERROR, "Failed to read audio file: {}", selected_file);
                matcher.send_text("读取电棍语录失败").await;
                return;
            }
        };
        matcher
            .send(
                UniMessage::new()
                    .record(FileType::Base64(audio_base64))
                    .build(),
            )
            .await;
    }

    async fn init(&self) {
        let mut file_paths = self.file_paths.lock().await;
        let entries = std::fs::read_dir(&self.path);
        match entries {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(path_str) = path.to_str() {
                                file_paths.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
            Err(_) => {
                event!(Level::ERROR, "Failed to read directory: {}", self.path);
            }
        }
    }
}

pub fn otto_quotation() -> Matcher<MessageEvent> {
    Matcher::new(
        "otto_quotation",
        OttoQuotation {
            path: "./otto_quotation".to_string(),
            file_paths: Mutex::new(vec![]),
        },
    )
    .add_rule(rules::in_groups(vec![
        // "657065745".to_string(),
        "711674260".to_string(),
        // "904639279".to_string(),
    ]))
}
