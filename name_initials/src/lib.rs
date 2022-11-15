pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| modify_name(name))
        .collect::<Vec<String>>()
}

fn modify_name(name: &str) -> String {
    name
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|word| {
                      let mut init = word
                                        .chars()
                                        .nth(0)
                                        .unwrap()
                                        .to_string();
                      init.push('.');
                      init
                    }
              )
        .collect::<Vec<String>>()
        .join(" ")
}