use std::borrow::Borrow;
trait FizzBuzzElement<O: Borrow<str>> {
    fn produce_fizzbuzz_word(i: usize) -> Option<O>;
}

macro_rules! generate_fizzbuzz {
    ($($elm: ident);*) => {
        struct FizzBuzz {
            index: usize
        }
        impl FizzBuzz {
            fn new(start_index: usize) -> Self {
                FizzBuzz {
                    index: start_index
                }
            }
        }
        impl Default for FizzBuzz {
            fn default() -> Self {
                FizzBuzz {
                    index: 0
                }
            }
        }
        impl Iterator for FizzBuzz {
            type Item = String;
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let mut element_products:Vec<Option<&str>> = vec![];
                $(element_products.push($elm::produce_fizzbuzz_word(n)));*;
                let result = element_products.iter().fold(None as Option<String>, |old, elm_prod| 
                    match elm_prod {
                        Some(elm_val) => match old {
                            Some(old_val) => Some(old_val + elm_val),
                            None => Some(elm_val.to_string()) 
                        },
                        None => old
                    }
                );
                match result {
                    Some(val) => Some(val),
                    None => Some(format!("{}",n))
                }
            }
            fn next(&mut self) -> Option<Self::Item> {
                let output = self.nth(self.index);
                self.index += 1;
                output
            }
        }
        impl DoubleEndedIterator for FizzBuzz {
            fn next_back(&mut self) -> Option<Self::Item> {
                let output = self.nth(self.index);
                self.index -= 1;
                output
            }
            
        }
    }
}

struct Fizz {}
impl FizzBuzzElement<&'static str> for Fizz {
    fn produce_fizzbuzz_word(i: usize) -> Option<&'static str> {
        if i % 3 == 0 {
            Some("Fizz")
        } else {
            None
        }
    }
}

struct Buzz {}
impl FizzBuzzElement<&'static str> for Buzz {
    fn produce_fizzbuzz_word(i: usize) -> Option<&'static str> {
        if i % 5 == 0 {
            Some("Buzz")
        } else {
            None
        }
    }
}

struct Baz {}
impl FizzBuzzElement<&'static str> for Baz {
    fn produce_fizzbuzz_word(i: usize) -> Option<&'static str> {
        if i % 7 == 0 {
            Some("Baz")
        } else {
            None
        }
    }
}

generate_fizzbuzz!(Fizz; Buzz; Baz);

fn main() {
    for elm in FizzBuzz::default() {
        println!("{}",elm);
    }
}
