#![feature(trivial_bounds)]

use std::borrow::Cow;
trait FizzBuzzElement {
    type NewArg;
    fn new(arg: <Self as FizzBuzzElement>::NewArg) -> Self;
    fn produce_fizzbuzz_word(&mut self, i: usize) -> Option<Cow<str>>;
}

macro_rules! generate_fizzbuzz {
    (struct $name: ident {$($index: ident: $elm: ty where NewArg = $arg: ty),*} ) => {
        struct $name {
            index: usize,
            gens: ($($elm),*)
        }
        impl $name {
            #[allow(dead_code)]
            fn new(start_index: usize, args: ($($arg),*)) -> Self {
                $name {
                    index: start_index,
                    gens: {
                        let ($($index),*) = args;
                        ($(<$elm as FizzBuzzElement>::new($index)),*)
                    }
                }
            }
        }
        #[allow(trivial_bounds)]
        impl $name where $($arg: Default),* {
            #[allow(dead_code)]
            fn new_default_args(start_index: usize) -> Self {
                Self::new(start_index, ($(<$arg as Default>::default()),*))
            }
        }
        impl Iterator for $name {
            type Item = String;
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let mut element_products:Vec<Option<Cow<str>>> = vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as FizzBuzzElement>::produce_fizzbuzz_word($index,n)));*
                }
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
        impl DoubleEndedIterator for $name {
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
    type NewArg = ();
    fn new(_: ()) -> Self {
        Fizz{}
    }
    fn produce_fizzbuzz_word(&mut self, i: usize) -> Option<Cow<str>> {
        if i % 3 == 0 {
            Some(Cow::Borrowed("Fizz"))
        } else {
            None
        }
    }
}

struct Buzz {}
impl FizzBuzzElement for Buzz {
    type NewArg = ();
    fn new(_: ()) -> Self {
        Buzz {}
    }
    fn produce_fizzbuzz_word(&mut self, i: usize) -> Option<Cow<str>> {
        if i % 5 == 0 {
            Some(Cow::Borrowed("Buzz"))
        } else {
            None
        }
    }
}

generate_fizzbuzz!{
    struct FizzBuzz {
        fizz: Fizz where NewArg = (),
        buzz: Buzz where NewArg = ()
    }
}

fn main() {
    for elm in FizzBuzz::new_default_args(0) {
        println!("{}",elm);
    }
}
