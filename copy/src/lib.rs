pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, to_exponent_func(c), to_natural_log(c))
}

pub fn str_function(a: String) -> (String, String) {
    let words: Vec<&str> = a
                            .split_whitespace()
                            .collect();

    let ans: String = words
                        .iter()
                        .map(|v| to_exponent_func(v.parse::<i32>().unwrap()).to_string())
                        .collect::<Vec<String>>()
                        .join(" ");

    (a, ans)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ans: Vec<f64> = b
                        .iter()
                        .map(|v| to_natural_log(*v))
                        .collect::<Vec<f64>>();
    
    (b, ans)
}

fn to_exponent_func(digit: i32) -> f64 {
    let float_value = digit as f64;
    float_value.exp()
}

fn to_natural_log(digit: i32) -> f64 {
    let float_value = digit as f64;
    float_value.abs().ln()
}