#![feature(trivial_bounds)]
use std::borrow::Cow;


macro_rules! simple_fizzbuzz_element {
    ( impl $name: ident {
        $nth_item: item
    } ) => {
        #[derive(Default)]
        struct $name {
            index: usize
        }
        impl ::core::iter::Iterator for $name {
            type Item = ::std::borrow::Cow<'static,str>;
            $nth_item
            fn next(&mut self) -> ::core::option::Option<::std::borrow::Cow<'static,str>> {
                let result = <Self as ::core::iter::Iterator>::nth(self,self.index);
                self.index += 1;
                result
            }
        }
        impl ::core::iter::DoubleEndedIterator for $name {
            fn next_back(&mut self) -> ::core::option::Option<::std::borrow::Cow<'static,str>> {
                let result = <Self as ::core::iter::Iterator>::nth(self,self.index);
                self.index += 1;
                result
            }
        }
    }
}

macro_rules! generate_fizzbuzz {
    (struct $name: ident<$lif: lifetime> {$($index: ident: $elm: ty),*} ) => {
        struct $name<$lif> {
            index: usize,
            gens: ($($elm),*),
            lifetime: ::core::marker::PhantomData<&$lif str>
        }
        impl<'s> $name<'s> {
            #[allow(dead_code)]
            pub fn new(start_index: usize, elements: ($($elm),*)) -> Self {
                $name {
                    index: start_index,
                    gens: {
                        let ($($index),*) = elements;
                        ($($index),*)
                    },
                    lifetime: ::core::marker::PhantomData
                }
            }
            #[allow(dead_code)]
            fn element_products_to_output(input: Vec<Option<Cow<str>>>, n: usize) -> String {
                input.iter().fold(None as Option<String>, |old, elm_prod| 
                    match elm_prod {
                        Some(elm_val) => match old {
                            Some(old_val) => Some(old_val + elm_val),
                            None => Some(elm_val.to_string()) 
                        },
                        None => old
                    }
                ).unwrap_or(format!("{}",n))
            }
        }
        #[allow(trivial_bounds)]
        impl<'s> Default for $name<'s> where $($elm: Default),* {
            #[allow(dead_code)]
            fn default() -> Self {
                Self::new(0, ($(<$elm as Default>::default()),*))
            }
        }
        impl<'s> Iterator for $name<'s> {
            type Item = String;
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let mut element_products:Vec<Option<Cow<str>>> = vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as ::core::iter::Iterator>::nth($index,n)));*
                }
                Some(Self::element_products_to_output(element_products,n))
            }
            fn next(&mut self) -> Option<Self::Item> {
                let mut element_products:Vec<Option<Cow<str>>> = vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as ::core::iter::Iterator>::next($index)));*
                }
                self.index += 1;
                Some(Self::element_products_to_output(element_products,self.index))
            }
        }
        impl<'s> DoubleEndedIterator for $name<'s> {
            fn next_back(&mut self) -> Option<Self::Item> {
                let mut element_products:Vec<Option<Cow<'s,str>>> = vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as ::core::iter::DoubleEndedIterator>::next_back($index)));*
                }
                self.index -= 1;
                Some(Self::element_products_to_output(element_products,self.index))
            }
            
        }
    }
}

simple_fizzbuzz_element!{
    impl Fizz {
        fn nth(&mut self, n: usize) -> Option<Cow<'static,str>> {
            if n % 3 == 0 {
                Some(Cow::Borrowed("Fizz"))
            } else {
                None
            }
        }
    }
}


simple_fizzbuzz_element!{
    impl Buzz {
        fn nth(&mut self, i: usize) -> Option<Cow<'static,str>> {
            if i % 5 == 0 {
                Some(Cow::Borrowed("Buzz"))
            } else {
                None
            }
        }
    }
}


generate_fizzbuzz!{
    struct FizzBuzz<'s> {
        fizz: Fizz,
        buzz: Buzz
    }
}

fn main() {
    for elm in FizzBuzz::default() {
        println!("{}",elm);
    }
}
