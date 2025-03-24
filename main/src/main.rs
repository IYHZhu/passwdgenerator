use anyhow::{bail, Result};
use clap::Parser;
use encryptor::password::generate_password;

/// 一个简单的适用于任何账号的密码生成器￼
#[derive(Parser, Debug)]
#[clap(version,about, long_about = None)]
struct Args {
    /// 用于生成密码的种子
    #[clap(short, long)]
    seed: String,

    /// 密码长度
    #[clap(short, long, default_value_t = 16)]
    length: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // 种子不能太短
    if args.seed.len() < 4 {
        bail!("seed {} length must >= 4", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);
    let passwd = generate_password(&seed[..], length);
    match passwd {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }

    Ok(())
}
