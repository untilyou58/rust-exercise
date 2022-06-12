// Input words from the keyboard then print out the number of these words appearing in the given string

fn count_occurrences(char: char, string: &str) -> i32 {
    let mut count = 0;
    for c in string.chars() {
        if c == char {
            count += 1;
        }
    }
    count
}

pub fn run() {
    let str = "The quick brown fox jumps over the lazy dog";
    let mut words = vec![];
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    for c in str.chars() {
        if c.is_alphabetic() {
            word.push(c);
        } else {
            if !word.is_empty() {
                words.push(word);
                word = String::new();
            }
        }
    }

    if !word.is_empty() {
        words.push(word);
    }

    let mut count = 0;
    for word in words {
        let word_count = count_occurrences(word.chars().next().unwrap(), str);
        if word_count > 1 {
            count += word_count;
        }
    }
    println!("{}", count);
}