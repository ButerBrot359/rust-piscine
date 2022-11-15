pub fn first_subword(mut s: String) -> String {
    let mut answer = String::new();

    for (i, v) in s.chars().enumerate() {
        if i == 0 {
            answer.push(v);
            continue
        }
        match v as u8 {
            64..=90 | 95 => { return answer },
            _ => answer.push(v as char)
        }
    }
    answer
}
