use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()_+=-[]\\{}|;':\",./<>?";

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
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }
    if lower {
        chars.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz");
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }
    if number {
        chars.extend_from_slice(b"0123456789");
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }
    if symbol {
        chars.extend_from_slice(b"!@#$%^&*()_+=-[]\\{}|;':\",./<>?");
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("Failed to choose a character");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    println!("{}", password);

    let estimate = zxcvbn(&password, &[])?;

    eprintln!("paasword strength: {}", estimate.score());

    Ok(())
}
