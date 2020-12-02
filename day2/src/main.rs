use text_io::try_read;

fn parser() -> Result<bool, String> {
    let min_result: Result<usize, _> = try_read!("{}-");
    if min_result.is_err() {
        return Err("EOF".to_string());
    }

    let min: usize   = min_result.unwrap();
    let max: usize   = try_read!("{} ").unwrap();
    let chr: String  = try_read!("{}: ").unwrap();
    let pass: String = try_read!("{}\n").unwrap();

    let num_chr = pass.split(&chr).count() - 1;
    Ok(min <= num_chr && max >= num_chr)
}

fn main() {
    let mut count = 0;
    loop {
        match parser() {
            Ok(b) => {
                if b {
                    count += 1;
                }
            }
            Err(_) => break,
        }
    }
    println!("{}", count);
}
