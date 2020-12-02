use text_io::try_read;

fn parser() -> Result<bool, String> {
    let min_result: Result<usize, _> = try_read!("{}-");
    if min_result.is_err() {
        return Err("EOF".to_string());
    }

    let mut i1: usize = min_result.unwrap();
    let mut i2: usize = try_read!("{} ").unwrap();
    let chr: char     = try_read!("{}: ").unwrap();
    let pass: String  = try_read!("{}\n").unwrap();

    //Make values 0-indexed
    i1 -= 1;
    i2 -= 1;

    let chr1 = pass.chars().nth(i1).unwrap() == chr;
    let chr2 = pass.chars().nth(i2).unwrap() == chr;

    //Substitute for logical XOR
    Ok(chr1 != chr2)
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
