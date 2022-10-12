struct PrimeNumbers {
    numbers: Vec<u32>,
}

impl PrimeNumbers {
    pub fn new(len: usize) -> Self {
        let mut prime_numbers: Vec<u32> = vec![2];

        Self::generate(len, &mut prime_numbers);

        Self {
            numbers: prime_numbers,
        }
    }

    fn generate(target: usize, data: &mut Vec<u32>) {
        let mut current = data[0];

        loop {
            current += 1;

            if data.len() == target {
                break;
            } else if current % 2 == 0 {
                continue;
            } else {
                // Check if currrent % (some value in data) != 0
                let mut is_prime = true;

                for &number in data.iter() {
                    if current % number == 0 {
                        is_prime = false;
                        break;
                    }
                }

                if is_prime {
                    print!("current prime number: {}\r", current);
                    data.push(current);
                }
            }
        }

        println!("\nDone!");
    }
}

impl std::fmt::Display for PrimeNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::from("[");

        for number in self.numbers.iter() {
            if number != self.numbers.last().unwrap() {
                output = format!("{} {},", output, *number);
            } else {
                output = format!("{} {} ]", output, *number);
            }
        }

        writeln!(f, "{}", output)
    }
}

fn main() {
    let prime_numbers = PrimeNumbers::new(69420);

    println!("{}", prime_numbers);
}
