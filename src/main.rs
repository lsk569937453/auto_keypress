#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;
mod auto;
use crate::auto::key_press::press;
use anyhow::Ok;
use clap::Parser;
use pinyin::{ToPinyin, ToPinyinMulti};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration}; // for write_all() // for read_to_end()

/// 自动按键
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 输入的txt文件路径
    #[arg(short, long)]
    pub path: String,
    /// 每次输入后休眠的时间
    #[arg(short, long)]
    pub sleep_second_million: Option<i32>,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if let Err(e) = read_file(args).await {
        println!("{}", e);
    }
    Ok(())
}
async fn read_file(args: Args) -> Result<(), anyhow::Error> {
    let path = args.path;
    let sleep_millon_second = args.sleep_second_million.unwrap_or(1000);
    let mut file = File::open(path).await?;

    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    let s = String::from_utf8(contents)?;
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec {
        let char_string = c.to_string();
        let char_str = char_string.as_str();
        let mut pinyin_iterate = char_str.to_pinyin().flatten().peekable();
        if pinyin_iterate.peek().is_none() {
            press(c)?;
        } else {
            for pinyin in pinyin_iterate {
                for c in pinyin.plain().chars() {
                    press(c)?;
                }
                press(' ')?;
            }
        }
        sleep(Duration::from_millis(sleep_millon_second as u64)).await;
    }
    Ok(())
}
