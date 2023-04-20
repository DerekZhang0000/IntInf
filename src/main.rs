extern crate bitvec;

use bitvec::prelude as bv;

struct DigitBits {
    bits: bv::BitVec,
}

impl DigitBits {
    fn new(digit: u8) -> Self {
        assert!(digit <= 9);

        let mut bitvec = bv::BitVec::new();
        for i in (0..4).rev() {
            bitvec.push((digit >> i) & 1 == 1);
        }

        return Self { bits: bitvec };
    }

    fn to_string(&self) -> String {
        let mut u8_int = 0u8;
        for bit in self.bits.iter() {
            u8_int <<= 1;
            if bit == true {
                u8_int |= 1;
            }
        }
        return u8::to_string(&u8_int);
    }
}

impl std::clone::Clone for DigitBits {
    fn clone(&self) -> Self {
        let mut cloned_bits = bv::BitVec::new();
        for bit in self.bits.iter() {
            cloned_bits.push(*bit);
        }

        return Self { bits: cloned_bits };
    }
}

struct int_inf {
    digits: Vec<DigitBits>,
    negative: bool,
}

impl int_inf {
    fn new(num_str: &str) -> Self {
        let mut digits = Vec::new();
        let mut chars = num_str.chars();
        let negative = num_str.starts_with('-');

        if negative {
            chars.next();
        }

        for ch in chars {
            digits.push(DigitBits::new(ch.to_digit(10).unwrap() as u8));
        }

        return Self {
            digits: digits,
            negative: negative,
        };
    }

    fn to_string(&self) -> String {
        let mut num_str = String::new();

        if self.negative {
            num_str.push('-');
        }

        for digit_bits in &self.digits {
            num_str.push_str(&digit_bits.to_string());
        }

        return num_str;
    }
}

impl std::fmt::Display for int_inf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::clone::Clone for int_inf {
    fn clone(&self) -> Self {
        let mut cloned_digits = Vec::new();
        for digit_bits in &self.digits {
            cloned_digits.push(digit_bits.clone());
        }

        return Self {
            digits: cloned_digits,
            negative: self.negative,
        };
    }
}

impl std::ops::Add for int_inf {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = Self {
            digits: Vec::new(),
            negative: false,
        };

        if (!self.negative && !other.negative) || (self.negative && other.negative)
        // Both positive or both negative
        {
            let mut carry: bool = false;
            let num_str_1 = self.to_string();
            let num_str_2 = other.to_string();
            let mut num_str_1_chars = num_str_1.chars();
            let mut num_str_2_chars = num_str_2.chars();
            let max_index = num_str_1.len().max(num_str_2.len());

            for index in 0..max_index {
                let mut digit_1 = 0u8;
                let mut digit_2 = 0u8;

                if let Some(ch) = num_str_1_chars.next_back() {
                    digit_1 = ch.to_digit(10).unwrap() as u8;
                }

                if let Some(ch) = num_str_2_chars.next_back() {
                    digit_2 = ch.to_digit(10).unwrap() as u8;
                }

                let mut digit_sum = digit_1 + digit_2;
                if carry {
                    digit_sum += 1;
                }

                carry = digit_sum > 9;
                if carry {
                    digit_sum -= 10;
                }

                result.digits.push(DigitBits::new(digit_sum));

                if index == max_index - 1 && carry {
                    result.digits.push(DigitBits::new(1));
                }
            }

            result.digits.reverse();

            if self.negative && other.negative
            // Sets negative flag to true if both are negative
            {
                result.negative = true;
            }

            return result;
        } else {
            // Only one addend is negative, flip the sign and treat as subtraction
            if self.negative {
                return other - int_inf::new(self.to_string().trim_start_matches('-'));
            } else {
                return self - int_inf::new(other.to_string().trim_start_matches('-'));
            }
        }
    }
}

impl std::ops::Sub for int_inf {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = Self {
            digits: Vec::new(),
            negative: false,
        };

        let num_str_1 = self.to_string();
        let num_str_2 = other.to_string();
        let mut num_str_1_chars = num_str_1.chars();
        let mut num_str_2_chars = num_str_2.chars();
        let max_index = num_str_1.len().max(num_str_2.len());

        let mut borrow: i8 = 0;
        for _ in 0..max_index {
            let mut digit_1 = 0i8;
            let mut digit_2 = 0i8;

            if let Some(ch) = num_str_1_chars.next_back() {
                digit_1 = ch.to_digit(10).unwrap() as i8;
            }

            if let Some(ch) = num_str_2_chars.next_back() {
                digit_2 = ch.to_digit(10).unwrap() as i8;
            }

            let mut digit_diff = digit_1 - digit_2 - borrow;

            if digit_diff < 0 {
                digit_diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.digits.push(DigitBits::new(digit_diff as u8));
        }

        result.digits.reverse();
        return result;
    }
}

impl std::ops::Mul for int_inf {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Self {
            digits: Vec::new(),
            negative: false,
        };

        let mut counter = int_inf::new("0");
        while counter.to_string() != other.to_string() {
            result = result + self.clone();
            counter = counter + int_inf::new("1");
        }

        return result;
    }
}

fn main() {
    let max_int = u64::max_value();
    let mut num = int_inf::new(max_int.to_string().as_str());
    let mut num2 = int_inf::new("1");
    let mut sum = num.clone() + num2.clone();
    println!("{} + {} = {}", num, num2, sum);

    // let min_int = u64::min_value();
    // num = int_inf::new(min_int.to_string().as_str());
    // num2 = int_inf::new("1");
    // sum = num.clone() - num2.clone();
    // println!("{} - {} = {}", num, num2, sum);

    let max_int = u64::max_value();
    num = int_inf::new(max_int.to_string().as_str());
    num2 = int_inf::new("71");
    sum = num.clone() * num2.clone();
    println!("{} * {} = {}", num, num2, sum);
}
