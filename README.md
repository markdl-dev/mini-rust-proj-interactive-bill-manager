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

### Remove Bills
- we want to be able to remove bills from the existing structure
- let’s now move to working with Hashmaps
    - import Hashmap from the standard library `std::collections::Hashmap`
        - update our `Bills` struct
            - change the `inner` field to `Hashmap`
                - key will be the name of the bill
        - update `Bills::new()`
            - change the inner field to a new Hahmap
                - `HashMap::new()`
        - update `Bills::add()`
            - change push (vector) to insert (HashMap)
            - insert(bill.name.to_string(), bill)
        - update `Bills::get_all()`
            - instead of iterating thru the entire collection, we’ll need to access the `inner` and use the `values()` function
                - which only accesses the values which are the bills and ignores the keys
                - and if we `collect()` those as a vector it should work
- let’s now create the remove bill function inside the `impl Bill`
    - add the `remove` function
    - takes in a mutable reference to self (`&mut self`) since we are making modifictions
    - takes in the key which is the name of the bill (name: &str)
    - we will return a bool to indicate whether or not the deletion was successful
    - call the `remove()`  function of the inner HashMap, using the name of the bill as key
        - the `remove()` function actually moves the value out of the HashMap completely and gives it back to us
        - for the type of the value we will actually get an Option and will look like this
            
            `let a: Option<Bill> = self.inner.remove(name)`
            
        - but since we do not need the value, and we just need to return a bool we will use the `.is_some()`
            
            `self.inner.remove(name).is_some()`
            
            - `true` if we have value
            - `false` if removing failed
- create the menu that will call the remove bill function
    - in the menu module `mod menu {}`
    - create a new function `remove_bill`
    - takes in a mutable reference to bills (`&mut Bills`)
    - we’ll need to display the bills to the user so they know which to delete
    - print a message to the user so they know what to do next “Enter bill name to remove”
    - get the name using the `get_input` function
        - if we get a name, we use it
        - if `None`, we just exit the function
    - we try to delete the bill using our `bills.remove` function
        - if it was successful we print a message “bill removed”
        - if it was unsuccessful we print a message “bill not found”
- integrate the remove menu to our main menu
    - add first a new variant of the remove bill in our MainMenu enum `MainMenu::RemoveBill`
    - on the `impl MainMenu`
        - add it on the `from_str` function to ‘3’
        - expose the menu to the user update the `show()` function
    - update our main menu loop, use the `MainMenu::RemoveBill and menu::remove_bills(&mut bills)`

```rust
use std::collections::HashMap;
use std::io;
```

```rust
pub struct Bills {
    inner: HashMap<String, Bill>,
}
```

```rust
impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
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
        bills.add(bill);

        println!("Bill Added");
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
        println!("Enter bill name to remove");

        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        if bills.remove(&name) {
            println!("bill removed")
        } else {
            println!("bill not found")
        }
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }
}
```

```rust
enum MainMenu {
    AddBill,
    RemoveBill,
    ViewBill,
}
```

```rust
impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::ViewBill),
            "3" => Some(MainMenu::RemoveBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!(" == Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Remove Bill");
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
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            None => return,
        }
    }
}
```

### Editing Bills
- update Bills struct impl
    - create a new function `update` to edit the bills
    - takes in a mutable reference to self, because we are changing a data that is within
    - takes in name of the bill
    - takes in the new amount of the bill
    - returns a `bool` so we know if the name was typed in correctly
        - if name was found, bill gets updated and return true
        - if not found, we just return false
    - match on the `inner` Hashmap and use `get_mut()` function
        - `get_mut()`
            - gets a mutable reference on the item that we want to find
            - this is what we’ll use if we want to mutate an item that exists within a Hashmap
        - if bill is found `Some(bill)`, we get the bill and set the amount to the new amount and return true
        - if the bill is not found `None`, we just return false
- next expose the update functionality to the menu
    - update menu module
        - add new function `update_bill`
            - takes in a mutable reference to bills cause we are making changes
            - display the bills, so the user knows which bill to update
            - print message “Enter bill to update”
            - get the name from the user using get_input()
                - if there is a name, we use it
                - if there is no name, we `return` and exit out of the function
            - get the amount from the user using the `get_bill_amount()` function
                - if there is an amount, we use it
                - if there is no amount or incorrect, we `return` and exit out of the function
            - we try to update the bill using the `bills.update()` function
                - if successful we print “updated”
                - if not and did not find the bill with the name, we print “bill not found”
- next integrate this menu to our main menu
    - add a new variant to our enum `MainMenu`
        - `UpdateBill`
    - add it on the `match` arm of the `MainMenu` `from_str()`
    - display it to the user on the `MainMenu` `show()`
    - add the option to the `match` arm on the `MainMenu` loop in the `main` function

```rust
impl Bills {
    ...

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                return true;
            }
            None => return false,
        }
    }
}
```

```rust
mod menu {
		...

    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }

        println!("Enter bill to update");

        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        let amount = match get_bill_input() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) == true {
            println!("updated")
        } else {
            println!("bill not found")
        }
    }
}
```

```rust
enum MainMenu {
    ...
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            ...
            "4" => Some(MainMenu::UpdateBill),
            _ => None,
        }
    }

    fn show() {
       ...
        println!("4. Update Bill");
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
                ...
                Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
                None => return,
            }
        }
    }
```

### Fix the error message when exiting the app
> thread 'main' panicked at 'no data entered', src/bin/bill_manager.rs:8:38                │
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
> 
- what we can do here is to add the question mark operator ? on the `get_input()` since it returns an option
    - but the ? question mark operator only works if the containing function e.g. (main) also returns an `Option`
    - since this is a `main` function and it does no return anything, we cannot use the question mark ? operator here

```rust
fn main() {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let user_input = get_input().expect("no data entered");
        match MainMenu::from_str(&user_input) {
		...
```

- what we can do is to create another function that returns an option and just move everything into it
    - create new function `run_program`
        - returns an `Option` with just a unit type. `Option<()>`
            - unit type - just means nothing
        - copy out all the code in the main function
        - in the `get_input()` function instead of just .`expect()` (which terminates the program when user gets error), we now can use the question mark operator ? `get_input()?`
            - question mark operator ? will extract the data from the `get_input()` and place it on the variable
            - if the user did not enter anything it will just return the function with `None`, then it will be fine
        - in the `match` arm selection Menu, when the user enters something invalid, instead of just returning and bailing out of the program we just exit out of the loop with `break` then the function will ends and the program will exit.
        - in the end of the function, just return `None`
    - on the main, just call `run_program()`

```rust
fn main() {
    run_program();
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let user_input = get_input()?;
        match MainMenu::from_str(&user_input) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}
```
