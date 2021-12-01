//! A frankly overengineered implementation of FizzBuzz
//! 
//! I sometimes dump time into this project without
//! much regard for stability, for obvious reasons, so expect breaking changes anytime.

#![feature(trivial_bounds)]


/// Macro to generate the FizzBuzz Iterator
/// 
/// This macro generates a struct, which will
/// query all given FizzBuzz elements and compute the result
/// of a FizzBuzz game when queried using its methods.
/// 
/// The macro takes in a struct declaration (with no lifetime or type generics),
/// whose name will be used for the resulting struct.
/// 
/// All of the struct's "field" names will be used only internally, and must be unique amongst themselves.
/// the types of these "fields" must be the types of the FizzBuzz elements queried in the final struct.
/// 
/// The given elements must implement at least `Iterator<Item = Cow<'static,str>>`, and can also optionally all
/// implement DoubleEndedIterator to derive a DoubleEndedIterator implementation for this struct as well.
/// 
/// This struct implements a `::new(...)` method, which takes in one of each of the types declared in the macro.
/// 
/// This struct also implements `Default` if all given elements also implement `Default`.
/// 
/// ```
/// use std::borrow::Cow;
/// use overengineered_fizz_buzz as fizz_buzz;
/// # fizz_buzz::simple_fizzbuzz_element!{
/// #     impl Fizz {
/// #         fn nth(&mut self, n: usize) -> Option<Cow<'static,str>>{
/// #             self.index = n + 1;
/// #             let result = if n % 3 == 0 {
/// #                 Some(Cow::Borrowed("Fizz"))
/// #             } else {
/// #                 None
/// #             }
/// #             self.index += 1;
/// #             result
/// #         }
/// #     }
/// # }
/// # 
/// # fizz_buzz::simple_fizzbuzz_element!{
/// #     impl Buzz {
/// #         fn nth(&mut self, n: usize) -> Option<Cow<'static,str>>{
/// #             self.index = n;
/// #             let result = if n % 5 == 0 {
/// #                 Some(Cow::Borrowed("Buzz"))
/// #             } else {
/// #                 None
/// #             }
/// #             self.index += 1;
/// #             result
/// #         }
/// #     }
/// # }
/// 
/// fizz_buzz::generate_fizzbuzz! {
///     struct FizzBuzz {
///         fizz: Fizz, // Here, both Fizz and Buzz have been previously declared
///         buzz: Buzz  // and are both valid FizzBuzz Elements
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! generate_fizzbuzz {
    {struct $name: ident {$($index: ident: $elm: ty),*} } => {

        #[forbid(trivial_bounds)]
        struct $name where $($elm: ::core::iter::Iterator<Item = ::std::borrow::Cow<'static,::core::primitive::str>>),* {
            index: ::core::primitive::usize,
            gens: ($($elm),*)
        }

        #[allow(trivial_bounds)]
        impl $name where $($elm: ::core::iter::Iterator<Item = ::std::borrow::Cow<'static,::core::primitive::str>>),* {
            #[allow(dead_code)]
            pub fn new($($index: $elm),*) -> Self {
                $name {
                    index: 0,
                    gens: ($($index),*)
                }
            }
            #[allow(dead_code)]
            fn element_products_to_output(input: ::std::vec::Vec<::core::option::Option<::std::borrow::Cow<'static,::core::primitive::str>>>, n: ::core::primitive::usize) -> ::std::string::String {
                input.iter().fold(::core::option::Option::<::std::string::String>::None, |old, elm_prod| 
                    match elm_prod {
                        ::core::option::Option::Some(elm_val) => match old {
                            ::core::option::Option::Some(old_val) => ::core::option::Option::Some(old_val + elm_val),
                            ::core::option::Option::None => ::core::option::Option::Some(elm_val.to_string()) 
                        },
                        ::core::option::Option::None => old
                    }
                ).unwrap_or(format!("{}",n))
            }
        }
        #[allow(trivial_bounds)]
        impl ::core::default::Default for $name where $($elm: ::core::default::Default),* {
            #[allow(dead_code)]
            fn default() -> Self {
                Self::new($(<$elm as ::core::default::Default>::default()),*)
            }
        }
        #[allow(trivial_bounds)]
        impl ::core::iter::Iterator for $name where $($elm: ::core::iter::Iterator<Item = Cow<'static,::core::primitive::str>>),* {
            type Item = ::std::string::String;
            fn nth(&mut self, n: ::core::primitive::usize) -> ::core::option::Option<::std::string::String> {
                self.index = n;
                let mut element_products: ::std::vec::Vec<::core::option::Option<::std::borrow::Cow<'static,::core::primitive::str>>> = ::std::vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as ::core::iter::Iterator>::nth($index,n)));*
                }
                self.index += 1;
                ::core::option::Option::Some(Self::element_products_to_output(element_products,n))
            }
            fn next(&mut self) -> ::core::option::Option<::std::string::String> {
                let mut element_products:Vec<Option<::std::borrow::Cow<'static,::core::primitive::str>>> = vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as ::core::iter::Iterator>::next($index)));*
                }
                let result = Self::element_products_to_output(element_products,self.index);
                self.index += 1;
                ::core::option::Option::Some(result)
            }
        }
        #[allow(trivial_bounds)]
        impl DoubleEndedIterator for $name where $($elm: ::core::iter::Iterator<Item = Cow<'static, str>> + ::core::iter::DoubleEndedIterator),*{
            fn next_back(&mut self) -> ::core::option::Option<::std::string::String> {
                let mut element_products: ::std::vec::Vec<::core::option::Option<::std::borrow::Cow<'static,str>>> = ::std::vec![];
                {
                    let ($(ref mut $index),*) = self.gens;
                    $(element_products.push(<$elm as ::core::iter::DoubleEndedIterator>::next_back($index)));*
                }
                let result = Self::element_products_to_output(element_products,self.index);
                self.index -= 1;
                ::core::option::Option::Some(result)
            }
            
        }
    }
}

/// Helper macro for creating a simple struct and fizzbuzz implementation
/// 
/// This macro creates a struct, derives Default on it, and implements both
/// `Iterator<Item = Cow<'static,str>>` and `DoubleEndedIterator`,
///  given only a `fn nth(&mut self, n: usize)` implementation.
/// 
/// ```
/// use std::borrow::Cow;
/// use overengineered_fizz_buzz as fizz_buzz;
/// 
/// fizz_buzz::simple_fizzbuzz_element! {
///     impl ExampleElement {
///         fn nth(&mut self, n: usize) -> Option<Cow<'static,str>> {
///             self.index = n;
///             // Code Logic here!
///             self.index += 1;
///             Some(Cow::Borrowed("Hardcoded String for now"))
///         }
///     }
/// }
/// ```
/// 
/// While implementation of other `Iterator` methods through this macro
/// is possible with some modification, it is not supported currently.
/// 
/// Currently the only state stored inside the new struct is a `usize` index.
/// 
/// Implementations are expected to modify the `self.index` field to match
/// the given `n` value plus one by the end of the iteration.
/// 
#[macro_export]
macro_rules! simple_fizzbuzz_element {
    { impl $name: ident {
        $nth_item: item
    } } => {
        #[derive(::core::default::Default)]
        struct $name {
            index: ::core::primitive::usize
        }
        impl ::core::iter::Iterator for $name {
            type Item = ::std::borrow::Cow<'static,::core::primitive::str>;
            $nth_item
            fn next(&mut self) -> ::core::option::Option<::std::borrow::Cow<'static,::core::primitive::str>> {
                ::core::iter::Iterator::nth(self,self.index)
            }
        }
        impl ::core::iter::DoubleEndedIterator for $name {
            fn next_back(&mut self) -> ::core::option::Option<::std::borrow::Cow<'static,::core::primitive::str>> {
                let result = ::core::iter::Iterator::nth(self,self.index);
                self.index -= 2;
                result
            }
        }
    }
}