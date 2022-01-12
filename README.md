# Rust Mini Project: Interactive Bill Manager
- Its a mini project to solidify my understanding of the basic Rust concepts.
- This is an activity in the Rust Programming Course from the [ZTM Academy](https://academy.zerotomastery.io/p/learn-rust)

## About
- Menu driven command line application
    - Selects a menu by entering a number
    - Perform action based on which menu you are working with
- Makes use of:
    - Modules
    - Enums
    - Options
    - Result
    - Match
    - Structs
    - Iterators
- Advanced issues that will be tackled
    - ownership and mutability problems
- Good practice for:
    - basics of Rust
    - reading compiler errors
    - understanding ownership and mutability

![about](img/about.png)
![sample](img/sample.png)

## Summary
> Create a command line bills/expenses manager that runs
> interactively. This mini project brings together many of
> the concepts learn thus far into a single application.
>
> The user stories/requirements are split into stages.
> Fully implement each stage as a complete working program
> before making changes for the next stage. Leverage the
> compiler by using `cargo check --bin bill_manager` when changing
> between stages to help identify adjustments that need
> to be made.

## User Stories
- The idea is to complete the user stories
- Start with user story 1 completely before moving on to the next.

> User stories:
> * Stage 1:
>   - I want to add bills, including the name and amount owed.
>   - I want to view existing bills.
> * Stage 2:
>   - I want to remove bills.
> * Stage 3:
>   - I want to edit existing bills.
>   - I want to go back if I change my mind.

> Rust is a fantastic language for refactoring code that's why the project is structured into three stages.
> 
> Each stage will require a multitude of changes and you'll be able to use the compiler diagnostics to help you out when you need to change your code. 

## Tips
> Tips:
> * Use the loop keyword to create an interactive menu.
> * Each menu choice should be it's own function, so you can work on the
>   the functionality for that menu in isolation.
> * A vector is the easiest way to store the bills at stage 1, but a
>   hashmap will be easier to work with at stages 2 and 3.
