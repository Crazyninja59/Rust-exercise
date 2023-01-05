use std::collections::HashMap;
use std::io;
fn main() {
    let mut database:HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut text = String::new();
        io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
        let input:Vec<_> = text.trim().split(" ").collect();
        match input[0]{
            "Add" => {
                let data = database.entry(input[3].to_string()).or_insert(vec![]);
                data.push(input[1].to_string());
                
            }
            "Get" => {
                if input.len() == 1{
                    println!("Список сотрудников в компании: ");
                    for value in database.values(){
                        for name in value{
                            println!("{}", name);
                        }
                    }

                }
                else {
                    let result = database.get(input[1]).cloned();
                    match result {
                        Some(mut t) => {
                            t.sort();
                            println!("Список сотрудников из отдела {}: {:?}",input[1],t);
                        }
                        None => println!("Wrong key")
                    }
                }
            }
            _ => break

        }
        
    }
}
