fn main() {
    let numbers = [7, 28, -23, 0, -11];
    let mut highest_number = numbers[0];
    for number in numbers {
        if number > highest_number {
            highest_number = number;
        } else {}
    }
    println!("Highest value from the array: {}", highest_number);
}
