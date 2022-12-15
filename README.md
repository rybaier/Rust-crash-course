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
    - use arrow -> to identify return type 

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
