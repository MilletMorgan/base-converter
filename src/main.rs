fn main() {
    let number_to_convert: i32 = 156;
    let decimal_to_binary: String = convert_decimal_number_to_binary_base(number_to_convert);

    println!("{} in binary = {}", number_to_convert, decimal_to_binary);
}

pub fn convert_decimal_number_to_binary_base(mut number_to_convert: i32) -> String
{
    let mut result = "".to_owned();

    while number_to_convert != 0 {
        if number_to_convert % 2 == 0 {
            result += "0";
        } else {
            result += "1";
        };

        number_to_convert /= 2;
    }

    return result.chars().rev().collect::<String>();
}