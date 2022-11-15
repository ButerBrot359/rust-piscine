use regex::Regex;

pub fn arrange_phrase(phrase: &str) -> String {    
    let mut ans_arr = phrase
                        .split_whitespace()
                        .map(|x| String::from(x))
                        .collect::<Vec<String>>();

   
    for word in phrase.split_whitespace().collect::<Vec<&str>>().iter(){
        
    };

    

    String::from("123")
}

fn extract_int_and_index(word: &str) -> (String, usize) {
    let re = Regex::new("/(\d+)/").unwrap();

    for (ind, val) in word.chars().enumerate() {
        println!("{}", re.is_match(val));
    };

    (String::from("1"), 2 as usize)
}

