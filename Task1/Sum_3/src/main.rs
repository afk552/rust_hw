fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 100];
    let sum: i32 = numbers.iter().sum();

    println!("Сумма: {}", sum);
}
