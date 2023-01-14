use std::io;

fn main() {
    let dream: String = "dream".chars().rev().collect();
    let dreamer: String = "dreamer".chars().rev().collect();
    let erase: String = "erase".chars().rev().collect();
    let eraser: String = "eraser".chars().rev().collect();

    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let mut s: String = line1.trim().chars().rev().collect();

    loop {
        // println!("{}", s);
        if s.len() == 0 {
            println!("YES");
            break;
        } else if s.len() >= dream.len() && s[..dream.len()].eq(dream.as_str()) {
            // println!("found dream");
            s = s[dream.len()..].to_string();
        } else if s.len() >= erase.len() && s[..erase.len()].eq(erase.as_str()) {
            // println!("found erase");
            s = s[erase.len()..].to_string();
        } else if s.len() >= dreamer.len() && s[..dreamer.len()].eq(dreamer.as_str()) {
            // println!("found dreamer");
            s = s[dreamer.len()..].to_string();
        } else if s.len() >= eraser.len() && s[..eraser.len()].eq(eraser.as_str()) {
            // println!("found eraser");
            s = s[eraser.len()..].to_string();
        } else {
            println!("NO");
            break;
        }
    }
}
