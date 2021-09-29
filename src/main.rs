trait FizzBuzzElement {
    fn produce_fizzbuzz_word<'w>(i: usize) -> Option<&'w str>;
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
impl FizzBuzzElement for Fizz {
    fn produce_fizzbuzz_word<'w>(i: usize) -> Option<&'w str> {
        if i % 3 == 0 {
            Some("Fizz")
        } else {
            None
        }
    }
}

struct Buzz {}
impl FizzBuzzElement for Buzz {
    fn produce_fizzbuzz_word<'w>(i: usize) -> Option<&'w str> {
        if i % 5 == 0 {
            Some("Buzz")
        } else {
            None
        }
    }
}

generate_fizzbuzz!(Fizz; Buzz);

fn main() {
    for elm in FizzBuzz::default() {
        println!("{}",elm);
    }
}
