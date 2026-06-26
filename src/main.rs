fn main() {
    let grades: [f64; 6] = [6.0, 8.5, 4.9, 1.5, 9.5, 9.0];
    let mut average: f64 = grades.iter().sum();
    average = average / 6.0;
    // needs an average of at least 7 to be approved
    if average >= 7.0 {
        println!("Approved with an average of {:.2}!", average);
    } else {
        println!("Failed with an average of {:.2}!", average);
    }
}
