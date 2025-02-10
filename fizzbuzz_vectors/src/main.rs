fn main() {
    let mut fizz_numbers: Vec<i32> = Vec::new();
    let mut buzz_numbers: Vec<i32> = Vec::new();
    let mut fizzbuzz_numbers: Vec<i32> = Vec::new();
    let mut other_numbers: Vec<i32> = Vec::new();

    for i in 1..31 {
        if i % 3 == 0 && i % 5 == 0 {
            fizzbuzz_numbers.push(i);
        } else if i % 3 == 0 {
            fizz_numbers.push(i);
        } else if i % 5 == 0 {
            buzz_numbers.push(i);
        } else {
            other_numbers.push(i);
        }
    }

    println!("Fizz numbers: {:?}", fizz_numbers);
    println!("Buzz numbers: {:?}", buzz_numbers);
    println!("FizzBuzz numbers: {:?}", fizzbuzz_numbers);
    println!("Other numbers: {:?}", other_numbers);
}

