use crate::models::challenge::md5::{MD5HashcashInput, MD5HashcashOutput};
use crate::models::challenge::ChallengeTrait;

pub struct Md5 {
    complexity: u32,
    message: String,
}

impl ChallengeTrait for Md5 {
    type Input = MD5HashcashInput;
    type Output = MD5HashcashOutput;

    fn name() -> String {
        "Md5".to_string()
    }

    fn new(input: Self::Input) -> Self {
        Md5 {
            complexity: input.complexity,
            message: input.message,
        }
    }

    fn solve(&self) -> Self::Output {
        for i in 0..std::u64::MAX {
            let seed_size = 16;
            let seed = format!("{i:0seed_size$X}");

            let hashcode_size = 32;
            let hashcode = format!(
                "{:0hashcode_size$X}",
                md5::compute(("".to_owned() + &seed + &self.message).into_bytes())
            );

            let seed_u64 = u64::from_str_radix(&seed, 16).unwrap();

            let current_answer = MD5HashcashOutput {
                seed: seed_u64,
                hashcode,
            };
            if self.verify(&current_answer) {
                return current_answer;
            }
        }
        MD5HashcashOutput {
            seed: 0,
            hashcode: "".to_string(),
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let hex_str = answer.hashcode.as_str();

        let mut zero_count = 0;
        let mut only_zeros = true;

        for char in hex_str.chars() {
            let hex_int = u32::from_str_radix(&char.to_string(), 16).unwrap();
            let binary = format!("{:04b}", hex_int);

            for b in binary.chars() {
                if b == '0' {
                    zero_count += 1;
                } else {
                    only_zeros = false;
                    break;
                }
            }
            if !only_zeros {
                break;
            }
        }

        zero_count >= self.complexity
    }
}
