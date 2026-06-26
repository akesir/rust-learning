fn main() {
    let height: f64 = 1.80;
    let weight: f64 = 70.5;
    let bmi: f64 = weight / (height * height);
    println!("Your BMI value is {:.1} kg/m²", bmi);
    if bmi < 18.5 {
        println!("That means you're underweight!");
    } else if bmi > 25.0 {
        println!("That means you're overweight!");
    } else {
        println!("That means you're healthy!");
    }
}
