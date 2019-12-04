//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let mut matching_passwords = vec![];
    for n in 153517..630395 {
        if is_valid_pt_1(n) {
            matching_passwords.push(n);
        }
    }
    dbg!(matching_passwords.len());
    let mut matching_passwords_pt2 = vec![];
    for n in 153517..630395 {
        if is_valid_pt_2(n) {
            matching_passwords_pt2.push(n);
        }
    }
    dbg!(matching_passwords_pt2.len());
    Ok(())
}

pub fn is_valid_pt_1(n: i32) -> bool {
    let s = n.to_string();
    // has at least one double
    let characters: Vec<i32> = s.chars().map(|e| e.to_string().parse().unwrap()).collect();
    let mut has_double = false;
    for i in 0..characters.len() - 1 {
        let (a, b) = (characters[i], characters[i + 1]);
        if a == b {
            has_double = true;
            break;
        }
    }
    let mut increasing = true;
    for i in 0..characters.len() - 1 {
        let (a, b) = (characters[i], characters[i + 1]);
        if a > b {
            increasing = false;
            break;
        }
    }
    return has_double && increasing;
}

pub fn is_valid_pt_2(n: i32) -> bool {
    let s = n.to_string();
    // has at least one double
    let characters: Vec<i32> = s.chars().map(|e| e.to_string().parse().unwrap()).collect();
    let mut has_double = false;
    let mut previous_digit = characters[0];
    let mut count_of_matching = 1;
    for i in 0..characters.len() - 1 {
        let current_digit = characters[i + 1];
        if current_digit == previous_digit {
            count_of_matching += 1;
        } else {
            if count_of_matching == 2 {
                break;
            }
            count_of_matching = 1;
            previous_digit = current_digit;
        }
    }
    if count_of_matching == 2 {
        has_double = true;
    }
    let mut increasing = true;
    for i in 0..characters.len() - 1 {
        let (a, b) = (characters[i], characters[i + 1]);
        if a > b {
            increasing = false;
            break;
        }
    }
    return has_double && increasing;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_valid_pt_1(122345));
        assert_eq!(false, is_valid_pt_1(122340));
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(true, is_valid_pt_2(122345));
        assert_eq!(false, is_valid_pt_2(122340));
        assert_eq!(false, is_valid_pt_2(222345));
        assert_eq!(true, is_valid_pt_2(222335));
        assert_eq!(false, is_valid_pt_2(111111));
        assert_eq!(false, is_valid_pt_2(111110));
        assert_eq!(true, is_valid_pt_2(111122));
    }
}
