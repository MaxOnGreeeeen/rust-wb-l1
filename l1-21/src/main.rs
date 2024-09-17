use std::{cmp::max, io, ops::Add};

fn main() -> Result<(), io::Error> {
    let mut buffer = String::new();

    println!("Введите два числа через пробел: ");

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    let buf_numbers = buffer.trim().split(" ").collect::<Vec<&str>>();

    let number_a = BigInt::new(buf_numbers[0]);
    let number_b = BigInt::new(buf_numbers[1]);

    let result = number_b + number_a;

    println!("{}", result.to_string());

    Ok(())
}

#[derive(Debug, Clone)]
struct BigInt {
    digits: Vec<u8>,
}
impl BigInt {
    pub fn new(number: &str) -> BigInt {
        BigInt {
            digits: number
                .chars()
                .rev()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }

    pub fn to_string(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|&d| char::from_digit(d as u32, 10).unwrap())
            .collect()
    }
}
// Реализация операции сложения для BigInt
impl Add for BigInt {
    type Output = BigInt;

    fn add(self, add_number: BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0;

        let max_len = max(self.digits.len(), add_number.digits.len());
        for i in 0..max_len {
            let a = *self.digits.get(i).unwrap_or(&0);
            let b = *add_number.digits.get(i).unwrap_or(&0);

            let sum = a + b + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt { digits: result }
    }
}
