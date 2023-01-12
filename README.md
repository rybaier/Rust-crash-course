# Ultimate Rust Crash Course with Nathan Stocks
# Ultimate Rust Crash Course 2: Intermediate Concepts with Nathan Stocks
All My source code for the Udemy Rust Courses with Nathan Stocks 

## My Course Notes
ALL STATEMENTS END IN SEMICOLONS!!!
Always import files into the lib.rs! Before adding them to main.rs
### Rust cli
- cargo run - runs file
- cargo new - creates new rust project
- cargo fmt - uses rustfmt to format the code.
- cargo clippy - checks for specific problems to write better idiomatic code
- cargo doc - for building documentation html for project 
    - cargo doc --no-deps --open 
    - cargo doc --no-deps --open --document-private-items
- cargo login - for registering access tokens
- cargo test - used for running tests. can run single test by providing direct path 
    - cargo test test::test_name_here
- cargo bench
#### Methods mentioned 
- iter()
- into_iter()
- clone()
- insert_str()
- match expression
    - require you to write a branch arm for every possible outcome
- is_some()
- is_none()
- unwrap()
- unwrap_or()
- expect()
- is_ok()
- is_err()
- thread::spawn()
- from_file()

###### Variables
 - use - let -  to initialize variable
    - variables are immutable by default
        - Safety, Concurrency, Speed
        - to make variable mutable
            - use - mut - 
            - ex: let bunnies = 12
                - let mut bunnies = 12
        - const can not be changed 
            - can be used glabally
                - inline to compile time
                - fast 
    - variables can be shadowed 

###### Scope
- no garbage collector 
- scope operator is double colons ::

###### Memory Safety
- as long as compiler can guarentee something is safe it will let you do it 

###### Functions
- define using - fn -
    - use snake_case
    - function parameters defined by name : type
    - use arrow -> to identify return type MUST DO THIS to gain a return value

##### Module System

###### Scalar Types
- integers
    - unsigned integers
        - use u for prefix
            - u8, u16, u32, u64, us128, usize
    - signed intergers
        - use i for prefix 
        - i8, i16, i32, i64, i128, isize
- floats
    - use f prefix
        - f32, f64
- booleans
    - specified by bool
        - true or false
- characters
    - specified by char

###### Compound Types
- Tuple 
    - store multiple values of any type
    - () with comma seperated values and types
        - ex: let info: (u8,f64,i32) = (1, 3.3, 999);
    - use dot syntax to access
    - or destructure
    - tuples have a maximum arrity of 12 
        - above 12 you lose functionality
    - arrity = how many items in tuple
- Array
    - multiple values of same type
    - specify type and number of values in array 
    - specify either 
        - literally - let buf = [1, 2, 3];
        - or with a value and how many you want - let buf = [0; 3];
        - types are always seperated with ; and you must define number of values - let buf [u8; 3] = [1, 2, 3];
    - limited to a size of 32 before losing functionality

- Vectors

###### Control Flow
- if expression
    - no ; after branch values
    - no return used 
- no terinary operator ? 
- loop 
    - unconditional loop
        - requires break statement to exit loop
        - breaking out of a nested loop
            - annotate beginning of first loop for which loop you want to break out of
                - annotate with single apostrophe at beginning 
                - ex 'bob: loop {
                    loop {
                        break 'bob'
                    }
                }
    - conditional loop
        - doesn't need a break statement
        - can use continue to repeat the loop
    - syntax loops
        - while 
        - for 
            - .iter() - iterates over values
            - can take a pattern
                - ex: 
                - let array = [(1, 2), (3, 4)]
                - for (x,y) in array.iter(){}
            - can chain options 
                - map, dot, etc
            - range 
                - start is inclusive end is exclusive 
                - for num in 0..50
                    - use 2 dots to specify range 
                - for num in 0..=50
                    - add equal sign to include end value
###### Strings
- str slice types
    - &str - borrowed string slice
        - can not be modified
        - subset of a string
        - valid UTF-8
    - String - acutal string slice
        - can be modified
        - valid UTF-8
###### Ownership
- makes informative compiler messages possible
- 3 rules to Ownership
    - Each value has an owner
    - there is only one owner
    - value gets dropped when the owner goes out of scope

###### References & Borrowing
- Lifetimes
    - rule that References must always be valid
- can be mutable 
    - syntax -  &mut
- dereference 
    - must be done manually
    - *s
- types can also be reference
- rules
    - either 1 mutable reference 
    - or
    - any number of immutable references

###### Structs
- can have 
    - data fields
    - methods
    - associated functions 
- syntax 
    - call with struct
        - in capital-camel case 
        - struct RedFox {
            enemy: bool,
            life: u8,
        }
    - methods and associated functions
        -  defined in implementation block
        - Self can be used to replace struct name inside of block
        - impl RedFox{
            fn new() -> Self {
                Self {
                    enemy: true,
                    life: 70,
                }
            }
        }

- no struct inheritance
###### Traits 
- competence over inheritance 
- trait Noisy {
    fn get_noise(&self)
}
- traits can be used on any struct
- special trait called Copy
    - can not be used with heaps
- traits can have inheritance
- fields can not be defined in traits 

###### Collections
- Vec <T, T>  - T = type
    - let army = Vec<Invader>
    - vector is a generic collection that holds a bunch of one type, and is useful for lists
    - when you create a vector you specify one tupe of object that it will store in angle brackets
    - you can push values into it
    - vectors act like a stack
        - so .push() appends to the end and .pop() removes the item from the end and returns it
    - store objects of known size next  to each other in memory
        - you can index into it
        - Rust will panic if index is out of bounds
    - use vec! as a prefix to create vectors from literal values   
- HashMap<K, V> - K= Key, V = Value
    - let mut h: HashMap<u8, bool> = HashMap:: new()
        - generic collection where you specify a type for the key and a type for the value
        - you access the values by key
        - in other languages called a dictionary
- other collection types
    - VecDeque 
        - uses a ring buffer to implement a double-ended queue, which can efficiently add or remove items from the front and the back. everything else is less efficient than a vector
    - LinkedList
        - quick at adding or removiing items at an arbitrary point in the list. 
    - HashSet
        - hashing implementation of a set that performs set operations really efficiently
    - BinaryHeap
        - like a priority queue which always pops off the max value
    - BTreeMap
        - alternate map implementation using a modified binary tree
    - BTreeSet
        - alternate set implementation using a modified binary tree

###### Enums
- Algebraic Data Types
- specify enum by 
    - keywork enum, name of the enum capital camel-case and the names of variants in a block
        - enum Color {
            Red, 
            Green,
            Blue,
        }
        - let color = Color::Red
- power of enum
    - comes from associating data and methods with the variants
    - can have a named variant with no data 
    - a variant can have a single type of data, a tuple of data, or an anonymous struct of data
    - it's like a union in C
- Option is a standard library generic enum
    - enum Option<T> {
        Some(T),
        None,
    }
    - read all the methods for option
- Result a generic enum in the standard library
    - used whenever somehting might have a useful result or might have an error
    - #[must_use]enum Result<T, E> {
        Ok(T),
        Err(E),
    }

###### Closures
- an anonymous function that can borrow or capture some data from the scope it is nested in
- the syntax is a parameter list between two pipes without type annotations followed by a block
    - |x,y| {x + y}
        - add(1,2); returns 3
        - the types of the arguments and the return calue are all inferred from how you use the arguments and what you return
- closures support move semantics 
    - let f = move || {}        
###### Threads
- Rust Threading is portable
    - works across Linux, Mac and Windows
- use std::thread at top of file


### Course 2 
###### Idiomatic 
- rustfmt
    - cargo fmt
- clippy
    - cargo clippy
        - style
        - correctness
        - Complexity
        - Performance
- attributes
    - #[allow(clippy::too_many_arguments)]
###### Documentation
- cargo doc
  - cargo doc --no-deps --open
    - --no-deps makes it so you only generate your library's documentation without
      generating all the documentation for all of your dependencies.
    - --open automatically opens the index page of the generated documentation in your default browser.
        - can also access by going to target/doc/packagename/index.html
- documentation comments are noted by 
    - line based documentation comments
        - /// then the comments 
    - Block based documentation comments
        - /**  **/
    - rest of the markdown features are the same except for a couple things
    - outer documentation comments
        - used for everything that inner documentation isnt used for
    - inner documentation comments
        - used for libraries and modules
        - //!
        - /*! !*/
    - structs
        - you document the struct as a whole at the top
        - the fields are documented serperately

###### Publishing 
- publishing a package on crates.io
    - publishing is PERMANENT!!!!!
    - Beware of passwords ssh keys or access tokens
    - don't publish useless packages 
    - package names need to be unique!
- how to publish   
    - login to crates.io
    - open drop down menu go to account settings and click on API access
        - create an access token 
        - run cargo login on machine and enter the access token you created
- package 
    - only a name and version are required to publish a package but should configure with the additional settings
- example 
    - [package]
    - name = "rusty_engine" 
    - version = "2.0.0"
    - description = "Learn Rust with a simple cross-platform, 2D game engine"
    - edition = "2021"
    - homepage = "https://github.com/CleanCut/rusty_engine" 
    - repository = "https://github.com/CleanCut/rusty_engine" 
    - readme = "README.md"
    - keywords = ["game", "engine", "graphics", "audio", "rusty"]
    - categories ["game-engines"]
    - license = "MIT OR Apache-2.0"

###### Iterators part II
- into_iter()
    - returns an iterator which takes ownership of the collection it is implemented on
    - ex. let v = vec![6, 7, 8];
        - v.into_iter().for_each(|num| println!("{}", num));
    - iterartors are typically faster than for loops
- iterator adaptors
    - tool that takes an iterator and outputs a different iterator
    - .map()
    - .filter()
    - into_iterator()
    - all iterator adaptors are documented
- iterator consumer 
    - consumes the final iterator causing the chain of iterator adaptors to to do their processing
    - .for_each()
    - .sum() needs to know type of value
    - .collect() - needs to know type of container
    - turbo fish ::<>
        - ex. .sum::<i32> 
- .iter_mut() - returns mutable references 
    - v.iter_mut() or for _ in &mut v
- .iter() - returns immutable references
    - v.iter() or for _ in v
- .into_iter() - consumes and returns owned items
    - v.into_iter() or for _ in v
- .drain() - takes different arguments depending on which collection you are looking at, in all cases it returns an iterator that takes ownership of some or all items in the collection, removing those items from the collection but leaves the collection intact so you can continue to use it 

###### Traits
- 4 things can implement a trait
    - structs
    - enums
    - closures 
    - functions
- Derivable traits
    - #[derive(Debug)]
    - Debug - debug formatting and Pretty Debug formatting
        - normal debug formatting attempts to create a one-line description of the struc or enum that looks similar to its definition
                - println!("{:?}", puzzle); // Normal Debug
                - println!("{:#?}", puzzle); // Pretty Debug
    - Clone - implementing this trait allows value to be cloned by calling the clone method 
    - Copy - if your type implements Copy then it will be copied instead of moved in move situations 
        - used for small values that fit entirely on the stack
            - integers, floats, booleans
            - heaps cannot implement Copy
- Implementing traits
    - 4 steps for implenting traits
        - Bring the Trait into the Scope
        - Boilerplate
        - Implentation
    - common traits
        - Default
            - can be derived
            - sets default values
        - PartialEq - partial equal
            - trait that does the actual calculations to test for equality
        - Eq - equal
            - marker trait that you can implement if the equality logic os reflexive, transitive and symmetric
        - From / Into
            - a pair of traits
            - From
                - if you implement From then Into automatically gets implemented for you
                - From< T > for U
            - Into
                - Into< U > for T
            - They describe the same transformation between two types but from different viewpoints
            - From < Puzzle > for String
            - Into < String > for Puzzle

###### Errors
- errors should be enums
- group errors - group your errors as variants of as few enums as makes sense
- only return your errors from your public library
    - exception is passing std library errors if it makes sense
-  include non-exhaustive
   - #[non-exhaustive]
- Implement Debug + Display + Error Traits in that order
    - Error is a sub trait of Debug and Display so you must implement those traits first before implementing the Error trait
- thiserror package/crate
    - a derive macro that will implement both the Display and Error traits
    - use thiserror::Error;
    - #[error("Error message")] goes in error enum
- Handling Errors
    - Non-Recoverable Errors
        - panic!()
        - .expect()
        - .unwrap()
    - Handle or return
        - if let blocks
        - match expressions
        - ? operator
        - anyhow package/crate
        - eyre package/crate
        - snafu package/crate

###### Unit Tests 
- about testing small units of code 
    - TDD 
- config attribute 
    - controls conditional compilation of the item it applies to. 
        - so we can choose when to compile specific modules when running tests
        - use super lib with asterick when writing tests
    - #[cfg(test)]
    - mod test {
        use super::*;
        #[test]
        test logic goes here
    }
    - assert_eq!() - takes 2 arguements that implement the Partial Eq trait fails if arguments are not equal
    - assert_ne!()- takes 2 arguments that implement Eq trait fails if arguments are equal
    - assert!() - forces you to do the comparison yourself to give the macro a boolean value. True passes, False fails the test 
    - panic! - panicing fails the test
        - unless you use #[should_panic]
- can optionally return a result

###### Integration Tests 
- require tests directory 
    - tests directory needs to be in root directory not in src 
- tests a bunch of functions and structs and things all together to test the integration of multiple components 

###### Benchmarks
- dont benchmark on public CI servers
- generate reports on runtime and outliers affecting runtime of files
- always start with idiomatic code, then identify something that is slow, and then try to optimize it and measure what happens
- Criterion 
    - goes in [dev-dependencies] in Cargo.toml file
    - import via use statment at top of benchmark file