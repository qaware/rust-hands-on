fn main() {
    let result = palindrom1("anna");

    print!("Result: {}", result);
}

// For-Schleife
fn palindrom1(input: &str) -> bool {
    for index in 0..input.len() {
        let char_from_start = input.chars().nth(index).unwrap();
        let char_from_back = input.chars().nth(input.len() - index - 1).unwrap();

        if char_from_start != char_from_back {
            return false;
        }
    }

    return true;
}

// Rekursion
fn palindrom2(input: &str) -> bool {
    if input.len() < 2 {
        return true;
    }

    let first_char = input.chars().nth(0).unwrap();
    let last_char = input.chars().nth(input.len() - 1).unwrap();

    if first_char != last_char {
        return false;
    }

    return palindrom2(&input[1..input.len() - 1]);
}

// String umdrehen und vergleichen
fn palindrom3(input: &str) -> bool {
    let reversed = input.chars().rev().collect::<String>();

    return reversed == input;
}