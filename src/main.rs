#[warn(dead_code)]
#[warn(unused_variables)]
fn roman_to_numeric(roman: &str) -> i32 {
    let mut total = 0;
    let mut previous_char = ' ';

    for value in roman.chars() {
        match value {
            'I' => {
                total += 1;
            }
            'V' => {
                total += 5;
                if previous_char == 'I' {
                    total -= 2;
                }
            }
            'X' => {
                total += 10;
                if previous_char == 'I' {
                    total -= 2;
                }
            }
            'L' => {
                total += 50;
                if previous_char == 'X' {
                    total -= 20;
                }
            }
            'C' => {
                total += 100;
                if previous_char == 'X' {
                    total -= 20;
                }
            }
            'D' => {
                total += 500;
                if previous_char == 'C' {
                    total -= 200;
                }
            }
            'M' => {
                total += 1000;
                if previous_char == 'C' {
                    total -= 200;
                }
            }
            _ => {}
        }
        previous_char = value;
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::roman_to_numeric;
    #[test]
    fn i_is_one() {
        assert_eq!(roman_to_numeric("I"), 1);
    }

    #[test]
    fn ii_is_two() {
        assert_eq!(roman_to_numeric("II"), 2);
    }

    #[test]
    fn iv_is_four() {
        assert_eq!(roman_to_numeric("IV"), 4);
    }

    #[test]
    fn v_is_five() {
        assert_eq!(roman_to_numeric("V"), 5);
    }

    #[test]
    fn x_is_nine() {
        assert_eq!(roman_to_numeric("IX"), 9);
    }

    #[test]
    fn x_is_ten() {
        assert_eq!(roman_to_numeric("X"), 10);
    }

    #[test]
    fn xiii_is_thirteen() {
        assert_eq!(roman_to_numeric("XIII"), 13);
    }

    #[test]
    fn xiv_is_fourteen() {
        assert_eq!(roman_to_numeric("XIV"), 14);
    }

    #[test]
    fn xix_is_nineteen() {
        assert_eq!(roman_to_numeric("XIX"), 19);
    }

    #[test]
    fn xl_is_forty() {
        assert_eq!(roman_to_numeric("XL"), 40);
    }

    #[test]
    fn l_is_fifty() {
        assert_eq!(roman_to_numeric("L"), 50);
    }

    #[test]
    fn xc_is_ninety() {
        assert_eq!(roman_to_numeric("XC"), 90);
    }

    #[test]
    fn c_is_hundred() {
        assert_eq!(roman_to_numeric("C"), 100);
    }

    #[test]
    fn cd_is_four_hundred() {
        assert_eq!(roman_to_numeric("CD"), 400);
    }

    #[test]
    fn d_is_five_hundred() {
        assert_eq!(roman_to_numeric("D"), 500);
    }

    #[test]
    fn dcc_is_700() {
        assert_eq!(roman_to_numeric("DCC"), 700);
    }

    #[test]
    fn cm_is_nine_hundred() {
        assert_eq!(roman_to_numeric("CM"), 900);
    }

    #[test]
    fn m_is_thousand() {
        assert_eq!(roman_to_numeric("M"), 1000);
    }

    #[test]
    fn mcm_is_1900() {
        assert_eq!(roman_to_numeric("MCM"), 1900);
    }

    #[test]
    fn mcmlxx_is_1970() {
        assert_eq!(roman_to_numeric("MCMLXX"), 1970);
    }

    #[test]
    fn mmxxii_is_2022() {
        assert_eq!(roman_to_numeric("MMXXII"), 2022);
    }
}

use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let value = roman_to_numeric(&args[1]);
    println!("{} is {}", &args[1], value);
}
