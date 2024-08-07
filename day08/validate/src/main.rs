use validator::{Validate, ValidationError};

fn test() {
    #[derive(Debug, Validate)]
    struct SignupData {
        #[validate(email)]
        mail: String,
        #[validate(url)]
        site: String,
        #[validate(
            length(min = 1, max = 10),
            custom(function = "validate_unique_username")
        )]
        first_name: String,
        #[validate(range(min = 18, max = 20, message = "Age must be between 18 and 20"))]
        age: u32,
    }
    fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
        if username == "xXxShad0wxXx" {
            return Err(ValidationError::new("terrible_username"));
        }
        Ok(())
    }

    let data = SignupData {
        mail: "test@example.com".to_string(),
        site: "https://www.example.com".to_string(),
        first_name: "abcd".to_string(),
        age: 100,
    };
    match data.validate() {
        Ok(_) => println!("Valid!"),
        Err(e) => {
            let errors = e.field_errors();
            for error in errors {
                // println!("Error: {:?} {:?}", error.0, error.1);
                for err in error.1 {
                    if let Some(msg) = &err.message {
                        println!("Error: {}", msg);
                    }
                }
            }
            // println!("Validation errors: {:?}", e.errors());
        }
    }
}
fn main() {
    test();
    println!("Hello, world!");
}
