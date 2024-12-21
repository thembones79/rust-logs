use std::{fs, io::Error};

fn main() {
    let text = fs::read_to_string("logs.txt");

    match text {
        Ok(text_that_was_read) => {
            println!("Text has {} characters.", text_that_was_read.len())
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed)
        }
    }

    // println!("{:#?}", text);

    // match divide(5.0, 3.0) {
    //     Ok(result_of_division) => {
    //         println!("{}", result_of_division)
    //     }
    //     Err(what_went_wrong) => {
    //         println!("{}", what_went_wrong)
    //     }
    // }
    //
    // match validate_email(String::from("aaaa@fff.pl")) {
    //     Ok(..) => println!("email is valid"),
    //     Err(reason_this_failed_validation) => {
    //         println!("{}", reason_this_failed_validation)
    //     }
    // }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cant divide by 0"))
    } else {
        Ok(a / b)
    }
}
