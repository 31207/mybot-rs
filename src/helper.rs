use builtin_plugins::matcher::prelude::*;
pub struct Helper {}

#[async_trait]
impl Handler<MessageEvent> for Helper {
    on_command!(MessageEvent, "help");

    async fn handle(&self, _: MessageEvent, matcher: Matcher<MessageEvent>) {
        matcher.send_text("这是help").await;
    }
}

pub fn helper() -> Matcher<MessageEvent> {
    Matcher::new("helper", Helper {})
        .add_rule(rules::is_superuser())
        .add_pre_matcher(prematchers::command_start())
        .add_rule(rules::is_group_message_event())
}
