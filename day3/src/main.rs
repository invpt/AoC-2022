use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(opt) = std::env::args().nth(1) else { panic!() };

    if opt == "1" {
        let stdin = std::io::stdin();

        let mut total_score = 0;
        loop {
            let mut line = String::new();
            stdin.read_line(&mut line)?;

            if line.is_empty() {
                break;
            }

            let a = &line[..line.len() / 2];
            let b = &line[line.len() / 2..];

            for ch in b.chars() {
                if a.contains(ch) {
                    total_score += score(ch);
                    break;
                }
            }

            println!("Total score so far: {total_score}");
        }

        Ok(())
    } else {
        let stdin = std::io::stdin();

        let mut total_score = 0;
        loop {
            let mut l1 = String::new();
            let mut l2 = String::new();
            let mut l3 = String::new();
            stdin.read_line(&mut l1)?;
            stdin.read_line(&mut l2)?;
            stdin.read_line(&mut l3)?;

            if l1.is_empty() {
                break;
            }

            for ch in l3.chars() {
                if l1.contains(ch) && l2.contains(ch) {
                    total_score += score(ch);
                    break;
                }
            }

            println!("Total score so far: {total_score}");
        }

        Ok(())
    }
}

fn score(ch: char) -> usize {
    if 'a' <= ch && ch <= 'z' {
        ch as usize - 'a' as usize + 1
    } else {
        ch as usize - 'A' as usize + 27
    }
}
