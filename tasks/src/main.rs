#![warn(clippy::all, clippy::pedantic)]

use core::f64;
use std::io::stdin;
use std::num::Wrapping;

fn main() {
    q1();
    q2();
    q3();
    q4();
    q5();
    q6();
    q7();
}

fn q1() {
    println!("What is 7 * 8?");

    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Error!");

    let trim_input = input.trim(); // пробелы и каретка

    if trim_input == "56" {
        println!("Correct!")
    } else {
        println!("Incorrect!")
    }
}

fn q2() {
    let x: u64 = 4_294_967_296;
    let y: u32 = x.try_into().expect("Failed!"); // работа с кастованием чисел

    if x == y as u64 {
        println!("x equals y.")
    } else {
        println!("x does not equal y.")
    }
}

fn q3() {
    let a: f64 = 5.5028236;

    println!("{a}")
}

fn q4() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let c: f64 = 0.3;
    let error_margin: f64 = f64::EPSILON; // погрешность по модулю

    println!("{error_margin}");

    if 0.1 + 0.2 == 0.3 {
        println!("Yes")
    } else {
        println!("No")
    }

    if (a + b - c).abs() < error_margin {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn q5() {
    let mut counter = Wrapping(0i8);
    let mut count: i8 = 0;
    // позволяет зациклить бесконечный цикл и двигаться от -128 до 128 и обратно в случае i8

    loop {
        println!("{counter}");
        counter += 1;

        count += count.checked_add(1).expect("Failed!")
        // - cпособ выхода при переполнении
    }
}

fn q6() {
    let s = "Привет!";

    println!("String {} has {} characters", s, s.len()); // len считает байты
    println!("String {} has {} characters", s, s.chars().count()) // считает буквы
}

fn call_me(n: u64, _: i32, c: u32) -> u64 {
    println!("{c}");
    n * 2
}

fn q7() {
    let one = 1;
    let c: i8 = -3;
    let n = call_me(one as _, 3, c as _);

    println!("N ----> {n}")
}
