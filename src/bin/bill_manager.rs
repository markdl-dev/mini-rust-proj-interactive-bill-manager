use std::io;

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
