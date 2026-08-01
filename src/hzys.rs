use crate::arg_parser::HuoziyinshuaCommand;
use builtin_plugins::matcher::prelude::*;
use clap::Parser;
use huoziyinshua_rs::Huoziyinshua;
use nonebot_rs::message::FileType;
use nonebot_rs::message::UniMessage;
use tokio::sync::Mutex;
use tracing::{Level, event};
pub struct HZYS {
    huoziyinshua: Mutex<Option<Huoziyinshua>>,
}

#[async_trait]
impl Handler<MessageEvent> for HZYS {
    on_start_with!(MessageEvent, "活字印刷");

    async fn handle(&self, event: MessageEvent, matcher: Matcher<MessageEvent>) {
        let cmd: Vec<&str> = event
            .get_raw_message()
            .trim()
            .split_ascii_whitespace()
            .collect();
        let huoziyinshua_command = if let Ok(cmd) = HuoziyinshuaCommand::try_parse_from(cmd) {
            cmd
        } else {
            matcher.send_text(format!("参数解析失败").as_str()).await;
            return;
        };
        if huoziyinshua_command.speed < 0.5 || huoziyinshua_command.speed > 2.0 {
            matcher.send_text("播放速度必须在0.5到2.0之间").await;
            return;
        }
        if huoziyinshua_command.sentence.is_empty() {
            matcher.send_text("输入的句子不能为空").await;
            return;
        }
        if huoziyinshua_command.sentence.chars().count() > 100 {
            matcher.send_text("输入的句子不能超过100个字符").await;
            return;
        }
        if let Some(huoziyinshua) = self.huoziyinshua.lock().await.as_mut() {
            let _ = huoziyinshua.generate(&huoziyinshua_command.sentence, true);
            let result = huoziyinshua.change_speed(huoziyinshua_command.speed);
            if huoziyinshua_command.reverse {
                let _ = huoziyinshua.reverse();
            }
            match result {
                Ok(_) => {
                    let wav_base64 = if let Ok(w) = huoziyinshua.save_and_get_wav_base64() {
                        w
                    } else {
                        matcher.send_text("获取音频文件base64失败").await;
                        return;
                    };

                    matcher
                        .send(
                            UniMessage::new()
                                .record(FileType::Base64(wav_base64))
                                .build(),
                        )
                        .await;
                }
                Err(err) => {
                    matcher
                        .send_text(format!("生成活字印刷失败: {}", err).as_str())
                        .await;
                }
            }
        } else {
            matcher.send_text("活字印刷未初始化").await;
            return;
        };
    }

    async fn init(&self) {
        let huoziyinshua = Huoziyinshua::new("./s");
        match huoziyinshua {
            Ok(h) => {
                *self.huoziyinshua.lock().await = Some(h);
            }
            Err(err) => {
                event!(Level::ERROR, "初始化活字印刷失败: {}", err);
            }
        }
    }
}

pub fn hzys() -> Matcher<MessageEvent> {
    Matcher::new(
        "hzys",
        HZYS {
            huoziyinshua: Mutex::new(None),
        },
    )
    .add_rule(rules::in_groups(vec![
        "657065745".to_string(),
        "711674260".to_string(),
        "904639279".to_string(),
    ]))
}
