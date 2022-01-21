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

## Development Steps

### Create a support function for managing user input
- create string buffer
- loop over the input using `io` module from the standard `std` library
    - if user does something wrong or if there’s an error in the terminal then we just loop over until we get some valid data
    - trim the whitespace on the terminal entry (when user press enter there’s going to be a new line at the end) from the `.read_line`
        - turn it to an owned string because we are returning an Optional owned String on our function
    - if input is empty return `None`, else we get the input which is an Option<String>
```rust
use std::io;

fn get_input() -> Option<String> {
    let mut buff = String::new();
    while io::stdin().read_line(&mut buff).is_err() {
        println!("Please enter your data again.");
    }
    let input = buff.trim().to_owned();
    if input == "" {
        return None;
    } else {
        return Some(input);
    }
}
```

### Create the main menu loop
- create an `Enum` for our main menu
    - add bill and view bill first for our first user story
- create a function for the our MainMenu enum to take input from the user and return a `MainMenu` variant
    - this will act as a check to see if our user enters a menu that is correct or incorrect.
    - if its correct we get the `Option<MainMenu>` back, else we get `None` for bad input
    - `match` on the `input` and let’s use a number system for selection
        - if they enter anything else, return `None`
- create another function to display the menu
- create the main menu loop in the main function
    - show the menu
    - get user input
        - for debugging purposes, we can use expect(). when the user hits enter with nothing the program will jut terminate with the message.
    - do a match on the from_str() when it takes in the user input, so we can check which option the user selected
        - for now we’ll use the `()` type when a valid Menu is selected
            - `()` type just means nothing. to be updated later
        - just return when invalid Menu is selected
```rust
enum MainMenu {
    AddBill,
    ViewBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bill");
    }
}

fn main() {
    loop {
        MainMenu::show();
        let user_input = get_input().expect("no data entered");
        match MainMenu::from_str(&user_input) {
            Some(MainMenu::AddBill) => (),
            Some(MainMenu::ViewBill) => (),
            None => return,
        }
    }
}
```

### Data Structures
- We want to be able to add bills with name and amount owed
- create Bill struct
    - name
    - amount
    - add derive with Debug and Clone traits
        - Debug - so we can easily print out the struct on the terminal
        - Clone - will allow us to make copies of the structure
- create Bills struct
    - inner - vector that contains bills
- implement functionality on the Bills struct
    - `new()` - creates new bills struct, set inner to an empty vector `vec![]`
    - `add(&mut self, bill: Bill)` - add bills
        - `&mut self`
            - takes in a mutable reference to self
                - so we can access `inner` value mutably, which means we can modify it
        - `bill: Bill`
            - takes in a bill
                - we take an owned Bill, we move this Bill to the `inner` `Bill` vector
        - push the bill to the vector
    - `get_all(&self)` `-> Vec<&Bill>`
        - takes a reference to `&self` so it can access the bills
        - return a new Vector that has reference to the existing bills
            - so we need to borrow the Bills, that way the calling function can print them without any issues
            - to get a Vector of the existing Bills in borrowed form(`&Bill`)
                - iterate over the Bills and `collect()` it
                - because when we call `iter()` it is automatically borrowed
    
```rust
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.push(bill)
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}
```

### Adding & Viewing Bills
- create the menus to expose the functionalities to the user
- create a new module named menu
    - because we know we will be having multiple menus
    - let's put all the menus in a single module so they are grouped together and easily accessible
    - import our structures and function inside the module
        - a module does not have access to things outside
- create a new function inside the module that will handle the add bill menu `add_bill`
    - set it as pub so we can access it outside the menu module
    - it will accept a mutable reference to a Bill structure `&mut Bills`
        - so we can add new Bills to the structure
    - print out message so the user knows what to enter
        - "Bill Name"
    - get the `name` using the `get_input()` function we defined earlier
        - do a `match` on it since it returns an Option
            - if we get input then return it, and that will get populated to the `name`
            - if we don't get input we just `return` out of the function with nothing
        - do the same for the `amount`
            - but since our amount is of type float, we will need to convert this. we will create a new function to do this. we will re-factor this later.
    - create the `Bill` and set the `name` and `amount`
        - if the name of the fields matches the name of the variables, we don't need to assign the field name.
    - add this new Bill to the bills structure
    - print out message "Bill Added"
- create a new function that will handle the bill amount input `get_bill_amount()`
    - returns an `Option<f64>` instead of a `String`
    - this will include the conversion of the `String` from `get_input()`
    - this will also be an infinite menu (so we ask again when user enters a wrong input)
    - print what needs to be entered here "Amount"
    - get the input using the `get_input()` function
        - do a `match` to get if there is an input
        - if there is no input return `None`
    - if our `&input` is empty or nothing, we also return `None`
    - parse the `String` to `f64`
        - we use `Result<f64, _>` and the `_` means we let Rust decide the error type.
        - we use `input.parse()`, `.parse()` will turn it into the appropriate data type for our example it will turn it into `f64`
        - do a `match` on the parsed input
            - if the Result returns Ok and have an amount, we return the amount as an option
            - if the Result returns an Err, we get an error and we ignore it cause we are not concern about the error message.
                - we create our own message that the user will just need to try again
- we need to modify our menu module
    - we need to import now this new function get_bill_amount()
    - update `add_bill` function
        - change the function that we used in amount to get the input. from `get_input()` to `get_bill_amount()` so the amount now will be an `f64` instead of a `String`
- next we create the menu for viewing the bills
    - we add a new function for the view bills menu in the menu module `view_bills`
        - takes in a reference to our `Bills` structure
        - no return type because we are just going to print information
        - loop through the bills using the `.get_all()` function which returns a vector references of bills
        - print the bill using the debug token `println!("{:?}", bill}` for now we just print out all the information of the bill.
- next we utilize these menus in our main menu loop
    - create a new bills structure using the `Bills::new()` we created
    - in the main menu loop, we now use our menu functions from the menu module

```rust
fn get_bill_input() -> Option<f64> {
    println!("Amount");

    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };

        if &input == "" {
            return None;
        }

        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(parsed_input) => return Some(parsed_input),
            Err(_) => println!("Please enter a number"),
        }
    }
}
```

```rust
mod menu {
    use crate::{get_bill_input, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name");

        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let amount = match get_bill_input() {
            Some(input) => input,
            None => return,
        };

        let bill = Bill { name, amount };
        bills.inner.push(bill);

        println!("Bill Added");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }
}
```

```rust
fn main() {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let user_input = get_input().expect("no data entered");
        match MainMenu::from_str(&user_input) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            None => return,
        }
    }
}
```
