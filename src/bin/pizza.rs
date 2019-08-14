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

fn list_of_pizzas() {

}

fn main() {
    let pm = Pizza::new("Margherita".to_string(), 6.10);
    let pa = Pizza::new("Diavola".to_string(), 8.34);
    let mut bask: Vec<Pizza> = Vec::new();
    bask.push(pm);
    bask.push(pa);
    //println!("{}", bask.get_sum());
    print_basket(&bask);
}
