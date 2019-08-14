use std::io;
//the file itself acts as a module: no need to mod declaration!
struct Pizza {
    name: String,
    price: f64
}

impl Pizza {
    fn new(name: String, price: f64) -> Self {
        Pizza {
            name: name,
            price: price
        }
    }

    fn name(self) -> String {
        self.name
    }

    fn price(self) -> f64 {
        self.price
    }

    fn description(self) -> String {
        format!("{} {}", self.name, self.price)
    }
}

fn print_basket(basket: &Vec<Pizza>) {
    println!("Basket------------");
    let mut tot = 0.0;
    for pizza in basket {
        println!("{} {}",pizza.name, pizza.price);
        tot = tot + pizza.price;
    }
    println!("Total Cost {}", tot);
}

fn list_of_pizzas(pizzas: &Vec<Pizza>) -> usize{
    println!("Please select a pizza to add to your basket:");
    let mut ix = 1;
    for pizza in pizzas {
        println!("{}: {}: {}", ix, pizza.name, pizza.price);
        ix = ix + 1;
    }

    let mut word = String::new();
    print!("Enter your choice here: \n");
    std::io::stdin().read_line(&mut word).expect("Failed to read the number!");
    let choice: usize = match word.trim().parse() {
        Ok(i) => i,
        Err(e) => 0,
    };
    choice
}

fn main() {
    let mut list: Vec<Pizza> = Vec::new();
    let pm = Pizza::new("Margherita".to_string(), 6.10);
    let pa = Pizza::new("Diavola".to_string(), 8.34);
    list.push(pm);
    list.push(pa);
    let mut bask: Vec<Pizza> = Vec::new();
    let mut a: usize = 1;
    while(a != 0) {
        a = list_of_pizzas(&list);
        print_basket(&bask);
        println!("{}", a);
    }
}
