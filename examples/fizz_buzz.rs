use std::borrow::Cow;
use overengineered_fizz_buzz as fizz_buzz;

// Implementation using the helper macro
fizz_buzz::simple_fizzbuzz_element!{
    impl Fizz {
        fn nth(&mut self, n: usize) -> Option<Cow<'static,str>>{
            self.index = n;
            let result = if n % 3 == 0 {
                Some(Cow::Borrowed("Fizz"))
            } else {
                None
            };
            self.index += 1;
            result
        }
    }
}

// Manual implementation  with the optional Default, ...
#[derive(Default)]
struct Buzz {
    index: usize
}

// ... the required `Iterator<Item = Cow<'static,str>>` ...
impl Iterator for Buzz {
    type Item = Cow<'static,str>;
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = n;
        let result = if n % 5 == 0 {
            Some(Cow::Owned("Buzz".to_string()))
        } else {
            None
        };
        self.index += 1;
        result
    }
    fn next(&mut self) -> Option<Self::Item> {
        let result = Self::nth(self, self.index);
        self.index += 1;
        result
    }
}

// ... and the optional DoubleEndedIterator.
impl DoubleEndedIterator for Buzz {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = Self::nth(self, self.index);
        self.index -= 1;
        result
    }
}


// Finally, we generate the struct that will compute the sequence with the provided macro
fizz_buzz::generate_fizzbuzz!{
    struct FizzBuzz {
        fizz: Fizz,
        buzz:Buzz
    }
}

// And we run it. Here we test iterator methods and the different ways of instantiating the final struct.
fn main() -> () {
    assert_eq!(FizzBuzz::new(Fizz::default(), Buzz::default())  .take(5).collect::<Vec<String>>(),         vec!["FizzBuzz","1","2","Fizz","4"]  );
    assert_eq!(FizzBuzz::default()                              .skip(5).take(5).collect::<Vec<String>>(), vec!["Buzz","Fizz","7","8","Fizz"]          );
}