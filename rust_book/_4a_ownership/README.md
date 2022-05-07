# OWNERSHIP
Main purpose is to manage heap data.

## Ownership Rules
- Each value in Rust has a variable that's call its ***owner***
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped.
# THE STACK AND THE HEAP
Both the stack and the heap are parts of memory available to your code to use at runtime.
They are structured in different ways.

### STACK
- Store values in the order it gets them.
- Remove the values in the opposite order (***last in, first out***)
- Adding data is called ***pushing onto the stack***
- Removing data is called ***popping off the stack***
- Data stored on the stack must have a ***fixed size***
- Pushing to the stack is faster than allocating on the heap
- Accessing data in the stack is faster than accessing data on the heap
### HEAP
- Data with unknown size at compile time
- Size that might change 
- Less organized
- Allocating space on the heap requires more work
- Accessing data on the heap is slower than on the stack
- Minimizing the amount of duplicate data on the heap
- Cleaning up unused data on the heap

>Put data on heap 
> -> the ***memory*** allocator finds an empty spot
> in the heap that is big enough 
> -> marks it as being used
> -> returns a ***pointer*** which is 
> the address of that location
> => ***Allocating on the heap***

Note:
>- Pointer to the heap is a ***known, fixed size*** => store the ***pointer*** on the ***stack***
>- Actual data, follow the ***pointer***

## USE CASE
### 1. Function
- Parameters, potentially, pointers to data on the heap
- Function's local variables get pushed onto the stack.
- When the function is over, those values get popped off the stack.