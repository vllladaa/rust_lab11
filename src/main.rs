fn is_palindrome(n: u32) -> bool {
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main() {
    let number = 12321;
    println!("Is {} a palindrome? {}", number, is_palindrome(number));
}