extern crate bitvec;
use bitvec::prelude as bv;

struct DigitBits
{
    bits: bv::BitVec,
}

impl DigitBits {
    fn new(digit: u8) -> Self
    {
        assert!(digit <= 9);

        let mut bitvec = bv::BitVec::new();
        for i in (0..4).rev()
        {
            bitvec.push((digit >> i) & 1 == 1);
        }

        return Self
        {
            bits: bitvec,
        }
    }

    fn to_string(&self) -> String
    {
        let mut u8_int = 0u8;
        for bit in self.bits.iter()
        {
            u8_int <<= 1;
            if bit == true
            {
                u8_int |= 1;
            }
        }
        return u8::to_string(&u8_int);
    }
}

struct int_inf
{
    digits: Vec<DigitBits>,
    negative: bool,
}

impl int_inf
{
    fn new(num_str: &str) -> Self
    {
        let mut digits = Vec::new();
        let mut chars = num_str.chars();
        let negative = num_str.starts_with('-');

        if negative {
            chars.next();
        }

        for ch in chars {
            digits.push(DigitBits::new(ch.to_digit(10).unwrap() as u8));
        }

        return Self
        {
            digits: digits,
            negative: negative,
        }
    }

    fn to_string(&self) -> String
    {
        let mut num_str = String::new();

        if self.negative
        {
            num_str.push('-');
        }

        for digit_bits in &self.digits
        {
            num_str.push_str(&digit_bits.to_string());
        }

        return num_str;
    }
}

impl std::fmt::Display for int_inf
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.to_string())
    }
}

impl std::clone::Clone for int_inf
{
    fn clone(&self) -> Self
    {
        let mut digits = Vec::new();
        for digit_bits in &self.digits
        {
            digits.push(digit_bits.clone());
        }

        return Self
        {
            digits: digits,
            negative: self.negative,
        };
    }
}

impl std::ops::Add for int_inf
{
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
        let mut result = Self
        {
            digits: Vec::new(),
            negative: false,
        };

        if (!self.negative && !other.negative) || (self.negative && other.negative) // Both positive or both negative
        {
            let mut carry: bool = false;
            let num_str_1 = self.to_string();
            let num_str_2 = other.to_string();
            let mut num_str_1_chars = num_str_1.chars();
            let mut num_str_2_chars = num_str_2.chars();
            let max_index = num_str_1.len().max(num_str_2.len());

            for index in 0..max_index
            {
                let mut digit_1 = 0u8;
                let mut digit_2 = 0u8;

                if let Some(ch) = num_str_1_chars.next_back()
                {
                    digit_1 = ch.to_digit(10).unwrap() as u8;
                }

                if let Some(ch) = num_str_2_chars.next_back()
                {
                    digit_2 = ch.to_digit(10).unwrap() as u8;
                }

                let mut digit_sum = digit_1 + digit_2;
                if carry
                {
                    digit_sum += 1;
                }

                carry = digit_sum > 9;
                if carry
                {
                    digit_sum -= 10;
                }

                result.digits.push(DigitBits::new(digit_sum));

                if index == max_index - 1 && carry
                {
                    result.digits.push(DigitBits::new(1));
                }
            }

            result.digits.reverse();

            if self.negative && other.negative  // Sets negative flag to true if both are negative
            {
                result.negative = true;
            }

            return result;
        }
        else    // Only one addend is negative
        {
            if self.negative
            {
                return other - self;
            }
            else
            {
                return self - other;
            }
        }
    }
}

impl std::ops::Sub for int_inf
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        let mut result = Self
        {
            digits: Vec::new(),
            negative: false,
        }
    }
}


fn main()
{
    let max_int = u64::max_value();
    let mut num = int_inf::new(max_int.to_string().as_str());
    let mut num2 = int_inf::new("1");
    let mut sum = num.clone() + num2.clone();
    println!("{} + {} = {}", num, num2, sum);
}
