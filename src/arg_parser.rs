use clap::Parser;
#[derive(Parser, Debug)]
pub struct HuoziyinshuaCommand {
    pub sentence: String,

    #[arg(
        long,
        short,
        default_value_t = 1.0,
        help = "播放速度，默认为1.0，范围0.5-2.0"
    )]
    pub speed: f32,

    #[arg(long, short, default_value_t = false, help = "是否反向播放")]
    pub reverse: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_args() {
        let args = vec![
            "huoziyinshua",
            "这是一个测试",
            "-s",
            "1.5",
            "-r",
        ];
        let command = HuoziyinshuaCommand::try_parse_from(args);
        match command {
            Ok(c) => {
                assert_eq!(c.sentence, "这是一个测试");
                assert_eq!(c.speed, 1.5);
                assert_eq!(c.reverse, true);
            }
            Err(err) => {
                panic!("Failed to parse arguments: {}", err);
            }
        }
    }
}
