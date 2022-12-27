use std::io;
fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("failed to read line");
    str = str.to_lowercase();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut new_str = String::new();
    for value in str.split_whitespace(){
        for b in value.chars(){
            let result = vowels.iter().find(|&&x| x == b);
            match result {
                None => {
                    let consonant_word = value[1..].to_owned()+ "-" + &b.to_string() + "ay" + " ";
                    new_str.push_str(&consonant_word);
                },
                _ => {
                    let vowel_word = value.to_owned()+ "-" + "hay" + " ";
                    new_str.push_str(&vowel_word);
                }
            }
            break;
        }
    }
    println!("{}",new_str);
}

