fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn fizz_buzz(numbers: &[i32]) {
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{num}: FizzBuzz");
        } else if num % 3 == 0 {
            println!("{num}: Fizz");
        } else if num % 5 == 0 {
            println!("{num}: Buzz");
        } else if is_even(num) {
            println!("{num}: Even");
        } else {
            println!("{num}: Odd");
        }
    }
}

fn sum_nums(numbers: &[i32]) {
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {sum}");
}

fn largest_num(numbers: &[i32]) {
    let mut largest: i32 = numbers[0];
    let mut i: usize = 1;

    loop {
        if i >= numbers.len() {
            break;
        }
        if numbers[i] > largest {
            largest = numbers[i];
        }
        i += 1;
    }

    println!("Largest number: {largest}");
}

fn main() {
    let numbers: [i32; 10] = [3, 8, 15, 22, 5, 30, 7, 12, 9, 10];

     fizz_buzz(&numbers);
     sum_nums(&numbers);
     largest_num(&numbers);
}

