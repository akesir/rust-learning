fn main() {
    const NUMBERS: [i32; 8] = [28, -26, -23, 7, 17, -23, -14, 0];
    println!("Positive numbers in the array: {}", positive_counter(&NUMBERS));
    println!("Negative numbers in the array: {}", negative_counter(&NUMBERS));
    println!("Zeros in the array: {}", zero_counter(&NUMBERS));

}

pub fn positive_counter(arr: &[i32]) -> i32 {
    let mut i = 0;
    for number in arr {
        if *number > 0 {
            i += 1;
        } else{}
    }
    return i;
}

pub fn negative_counter(arr: &[i32]) -> i32 {
    let mut i = 0;
    for number in arr {
        if *number < 0 {
            i += 1;
        } else{}
    }
    return i;
}

pub fn zero_counter(arr: &[i32]) -> i32 {
    let mut i = 0;
    for number in arr {
        if *number == 0 {
            i += 1;
        } else{}
    }
    return i;
}