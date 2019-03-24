use std::str::FromStr;

const DIGITS: &'static [&str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TENS: &'static [&str] = &["zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
const UNIONS: &'static [&str] = &["dummy", "hundred", "thousand", "million"];

pub fn convert_into_text(input: usize) -> String {
    if input < 0usize
    {
        return "zero".into();
    }
    if input < 20usize
    {
        return DIGITS[input].into();
    }
    // Result collector
    let mut words: Vec<String> = Vec::new();
    let mut final_suffix = "";
    let mut proceed = true;
    // Actual algorithm
    let mut digit_string = input.to_string();
    while proceed
        {
            match digit_string.len()
                {
                    0 => proceed = false,
                    9 | 6 | 3
                    => {
                        let index = digit_string.len() / 3;
                        let hundred = digit_string.pop_first_as_usize();
                        if hundred > 0
                        {
                            match index {
                                1 => final_suffix = "",
                                _ => final_suffix = UNIONS[index]
                            }
                            let data = format!("{} {}", DIGITS[hundred], UNIONS[1]);
                            words.push(data);
                        }
                    },
                    7 | 4
                    => {
                        let thousand = digit_string.pop_first_as_usize();
                        if thousand > 0
                        {
                            let part = match digit_string.len()
                                {
                                    6 => 3,
                                    _ => 2
                                };
                            let data = format!("{} {}", DIGITS[thousand], UNIONS[part]);
                            words.push(data);
                        }
                    },
                    8 | 5 | 2
                    => {
                        let after_pop_len = digit_string.len() - 2;
                        let current: usize = digit_string.pop_first_as_usize();
                        let next : usize = digit_string.pop_first_as_usize();
                        let union = "";
                        if current.eq(&1)
                        {
                            let index_union = usize::from_str(&format!("{}{}", current.to_string(), next.to_string())).unwrap_or(0);
                            let data = DIGITS[index_union];
                            words.push(data.to_owned());
                            continue;
                        }
                        if next.eq(&0)
                        {
                            if current.eq(&0)
                            {
                                continue;
                            }
                            words.push(TENS[current].to_owned());
                            continue;
                        }
                        let union = match after_pop_len
                            {
                                6 => UNIONS[3],
                                3 => UNIONS[2],
                                0 => "",
                                _ => panic!("Unexpected!")
                            };
                        let suffix = format!("-{}",DIGITS[next]);
                        let prefix = TENS[current];
                        let data = format!("{}{} {}", prefix, suffix, final_suffix);
                        words.push(data);
                    },
                    _ => panic!("number is too big!!!")
                }
        }
    // Return Joined result
    let joined_result = words.join(" ").trim().to_string();
    return joined_result;
}
///
/// Extension trait for popping first symbol of String as usize
///
pub trait Popper<T> {
    fn pop_first_as_usize(&mut self) -> usize;
}
///
/// Actual implementation
///
impl Popper<String> for String {
    fn pop_first_as_usize(&mut self) -> usize {
        let first = self.remove(0);
        first.to_digit(10).unwrap() as usize
    }
}

#[cfg(test)]
mod humanizer_tests {
    use super::*;
    #[test]
    fn amount_to_text()
    {
        assert_eq!(convert_into_text(123),
                   String::from_str("one hundred twenty-three").unwrap());

        assert_eq!(convert_into_text(999999),
                   String::from_str("nine hundred ninety-nine thousand nine hundred ninety-nine").unwrap());
    }
}