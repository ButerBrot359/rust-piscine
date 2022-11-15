pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for letter in v.chars() {
        if (letter as u16) > 255 {return false};
    };
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[0..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    match v.find(pat) {
        Some(val) => val,
        None => 0
    }
}