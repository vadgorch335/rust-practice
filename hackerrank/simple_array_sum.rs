use std::io;

fn main() {
    let mut input = String::new();

    // Зчитуємо кількість елементів (опціонально)
    io::stdin().read_line(&mut input).unwrap();
    input.clear(); // очищаємо строку для наступного вводу

    // Зчитуємо елементи масиву
    io::stdin().read_line(&mut input).unwrap();

    // Обробляємо введені числа, перетворюємо в i32 і обчислюємо суму
    let sum: i32 = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum();

    println!("{}", sum);
}
