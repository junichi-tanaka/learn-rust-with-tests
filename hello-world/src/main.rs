
const SPANISH: &'static str = "Spanish";
const FRENCH: &'static str = "French";
const ENGLISH_HELLO_PREFIX: &'static str = "Hello, ";
const SPANISH_HELLO_PREFIX: &'static str = "Hola, ";
const FRENCH_HELLO_PREFIX: &'static str = "Bonjour, ";

fn hello(name: &str, language: &str) -> String {
    let mut _name = name;
    let world = String::from("World");
    if _name == "" {
        _name = &world
    }

    return greeting_prefix(language) + _name
}

fn greeting_prefix(language: &str) -> String {
    if language == FRENCH {
        FRENCH_HELLO_PREFIX.to_string()
    } else if language == SPANISH {
        SPANISH_HELLO_PREFIX.to_string()
    } else {
        ENGLISH_HELLO_PREFIX.to_string()
    }
}

fn main() {
    println!("{}", hello("Chris", ""));
    println!("{}", hello("", ""));
    println!("{}", hello("Elodie", SPANISH));
    println!("{}", hello("Lauren", FRENCH));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello("Chris", ""), "Hello, Chris");
        assert_eq!(hello("", ""), "Hello, World");
        assert_eq!(hello("Elodie", SPANISH), "Hola, Elodie");
        assert_eq!(hello("Lauren", FRENCH), "Bonjour, Lauren");
    }
}
