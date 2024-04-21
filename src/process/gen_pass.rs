use rand::seq::SliceRandom;
extern crate zxcvbn;

use zxcvbn::zxcvbn;

const UPPER_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CHARS: &[u8] = b"abcdefghjklmnpqrstuvwxyz";
const NUMBER_CHARS: &[u8] = b"123456789";
const SYMBOL_CHARS: &[u8] = b"!@#*_&^";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER_CHARS);
        password.push(*UPPER_CHARS.choose(&mut rng).expect(""));
    }

    if lower {
        chars.extend_from_slice(LOWER_CHARS);
        password.push(*LOWER_CHARS.choose(&mut rng).expect(""));
    }

    if number {
        chars.extend_from_slice(NUMBER_CHARS);
        password.push(*NUMBER_CHARS.choose(&mut rng).expect(""));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL_CHARS);
        password.push(*SYMBOL_CHARS.choose(&mut rng).expect(""));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("不可能为空");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let pwd = String::from_utf8(password)?;

    println!("{:?}", pwd);

    let estimate = zxcvbn(&pwd, &[])?;
    eprintln!("password strength: {}", estimate.score());

    Ok(())
}
