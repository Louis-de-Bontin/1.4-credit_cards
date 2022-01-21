use std::io;


fn main() {
    let (input, compagny_name) = get_input();
    println!("Your card number is : {}", input);
    let digit_vec = num_digits(input);
    let mut i = 1;


    let mut intermediat_vec: Vec<u128> = Vec::new();    // Declare a new vector
    let revert_vector: Vec<u128> = digit_vec.iter().rev().cloned().collect();
    let mut sum_pair: u128 = 0;
    for digit in revert_vector {
        if i % 2 == 0 {
            intermediat_vec.push(digit*2);
        } else {
            sum_pair += digit;
        }
        i+=1;
    }
    let mut sous_tot: u128 = 0;
    for nb in intermediat_vec {
        sous_tot += (nb/10) + (nb%10);                  // Exemple : 37 -> 3+7; 6 -> 6; 98 -> 9+8
    }
    let tot = sous_tot + sum_pair;


    if tot % 10 == 0 {
        println!("Your card is valid ! It is a {} card.", compagny_name)
    } else {
        println!("Yout card is not valid !")
    }
}

fn get_input() -> (u128, String) {
    // This function get the user input and check if
    // the length is correct, the 2 first number are correct
    // and if there is only numbers.

    let mut input;

    loop {
        input = String::new();
        println!("What is yout card number ?");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        input = input.trim().parse().expect("Unable to parse value !");
        input = replace_char(input, ['-', '/', ' ']);

        let len = input.chars().count();
        let firsts_digits = get_firsts_digits(&input, 2);             // I had to return input as well otherwise it would be considered as "borrowed" and would not compile
        let valid_len = [13, 15, 16];

        if !!!valid_len.contains(&len) || firsts_digits == 11 {
            println!("This is not a valid card number !");
            continue;
        }
        let card_compagny = get_compagny_name(firsts_digits);
        if card_compagny == "unknown" {
            continue
        }

        
        let input: u128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a valid card number !");
                continue;
            },
        };
        return (input, card_compagny);
    }
}

fn get_compagny_name(firsts_char_of_card: u32) -> String {
    let valid_amex = [34, 37];
    let valid_mastercard = [51, 52, 53, 54, 55];
    let valid_visa = [40, 41, 42, 43, 44, 45, 46, 47, 48, 49];
    if valid_amex.contains(&firsts_char_of_card) {
        String::from("American Express")
    } else if valid_mastercard.contains(&firsts_char_of_card) {
        String::from("Mastercard")
    } else if valid_visa.contains(&firsts_char_of_card) {
        String::from("Visa")
    } else {
        String::from("unknown")
    }
}

fn get_firsts_digits(input: &String, nb_char: usize) -> u32 { 
    // This function take a string in input, and return the 2
    // first char if the string is long enough or 11 if it isn't
    // or if the 2 firsts char arn't digits.

    let char_vec: Vec<char> = input.chars().collect(); // Transform a string into a vector with every char of the string
    let mut out = String::new();
    if char_vec.len() <= 1 {
        let mut i = 0;
        while i < nb_char{                                   // I can't push a str, so I had to loop to push a char twice
            out.push('1');
            i+=1;                                      // I return an invalid value on purpose, the error is handled in get_input
        }
    } else {
        let mut i: usize = 1;
        out = String::from(char_vec[0]);               // Make a string out of the first char of the vector
        while i < nb_char {
            out.push(char_vec[i]);                         // Append the second char of the vector
            i += 1;
        }
    }

    let out: u32 = match out.trim().parse() {
        Ok(num) => num,
        Err(_) => 11,
    };

    return out;
}

#[allow(dead_code)]
fn num_digits(num: u128) -> Vec<u128> {                // Transform a u128 nb in a vector of each digit
    let mut x = num;
    let mut result: Vec<u128> = Vec::new();

    loop {
        result.push(x % 10);
        x /= 10;
        if x == 0 {break}
    }

    result.reverse();
    result
}

fn replace_char(a_string: String, forbiden_chars: [char; 3]) -> String {         // Delete specific char from String
    let mut new_string = String::from ("");
    // let forbiden_chars = ['-', '/', ' '];
    for char in a_string.chars() {
        if !!!&forbiden_chars.contains(&char) {
            new_string.push(char);
        }
    }
    new_string
}
