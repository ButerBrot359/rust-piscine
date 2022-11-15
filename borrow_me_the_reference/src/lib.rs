use evalexpr::{eval};

pub fn delete_and_backspace(s: &mut String) {
    let mut letter_stack = Vec::new();
    let mut delete_forward_count = 0;

    for v in s.chars() {
        match v {
            '+' => delete_forward_count += 1,
            '-' => {letter_stack.pop().unwrap(); ()},
            _ => {
                if delete_forward_count > 0 {
                    delete_forward_count -= 1;
                    continue
                }
                letter_stack.push(v)
            }
        }
    }
}

pub fn do_operations(v: &mut Vec<String>) {
    let ans = v
        .iter()
        .map(|expression| eval(expression).unwrap().to_string())
        .collect::<Vec<String>>();

    *v = ans;
}
