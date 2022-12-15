# Rust Crash Course with Nathan Stocks
My source code for the Udemy Rust Course with Nathan Stocks 

## My Course Notes
ALL STATEMENTS END IN SEMICOLONS!!!
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
    - 
- rules
    - either 1 mutable reference 
    - or
    - any number of immutable references



#### Methods discussed 
- iter()
- clone()
- insert_str()