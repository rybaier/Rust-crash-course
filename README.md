# Rust Crash Course with Nathan Stocks
My source code for the Udemy Rust Course with Nathan Stocks 

## My Course Notes
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
- 