fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize; // Довжина рядка
    let shift = (n % len + len) % len; // Обчислення зміщення з урахуванням довжини рядка
    let split_point = len as usize - shift as usize;

    // Об'єднуємо частини рядка після зміщення
    format!("{}{}", &s[split_point..], &s[..split_point])
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    // Використання for_each для проходження по елементах shifts
    shifts
        .iter()
        .for_each(|(n, exp)| {
            let result = rotate(s.clone(), *n);
            println!("Зміщення: {}, Результат: {}, Очікувано: {}", n, result, exp);
        });
}
