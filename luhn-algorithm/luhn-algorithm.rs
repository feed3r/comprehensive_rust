// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn check_empty_string(cc_number: &str) -> bool {
    return cc_number.trim().len() > 0;
}

fn check_string_only_digit(cc_number: &str) -> bool {
    return !cc_number.chars().any(char::is_alphabetic);
}

fn check_string_digits(cc_number: &str) -> bool {
    return cc_number.split(' ').collect::<Vec<&str>>().len() >= 2;
}

fn sum_digits(num:i32)->i32 {
	let mut sum:i32=0;
    let mut local_num = num;
	loop {
		sum = sum + (local_num % 10);
		local_num /= 10;
        if local_num == 0 {
            break;
        }
	}
	return sum;
}

fn validate_string_structure(cc_number: &str) -> bool {
    if  !check_empty_string(cc_number) {
        println!("Error in check empty string");
        return false;
    }
    if !check_string_digits(cc_number) {
        println!("Error in check string digits");
        return false;
    }
    if !check_string_only_digit(cc_number)
    {
        println!("Error in check string only digits");
        return false;
    }
    
    return true;
}

fn calculate_checksum(cc_number: &str) -> i32 {
    let mut checksum = 0;
    let mut cc_index = 0;

    for c in cc_number.chars().rev() { //moving from right to left
        if c == ' ' { //ignore spaces
            continue;
        }

        cc_index = cc_index + 1;

        let num = (c.to_string()).parse::<i32>().unwrap();
        if cc_index % 2 == 0 { //double every second digit 
            checksum += sum_digits(num * 2);
        } else {
            checksum += num;
        }
    }

    return checksum;
}

fn validate_checksum(cc_number: &str) -> bool {
    let checksum = calculate_checksum(cc_number) % 10;
    println!("Checksum calculated {}", checksum);
    checksum == 0
}

pub fn luhn(cc_number: &str) -> bool {
    if !validate_string_structure(cc_number) {
        println!("Failed structure check");
        return false;        
    }

    if !validate_checksum(cc_number) {
        println!("Failed checksum");
        return false;
    }

    return true;
}

#[test]
fn test_sum_digit() {
    let res = sum_digits(12345);
    println!("calculated {}", res);
    assert!(res == 15);
}


#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    println!("result {}", luhn(" 0 0 "));
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
}