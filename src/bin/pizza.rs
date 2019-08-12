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
}

trait Pizzable {
    fn get_price(&self) -> f64;
}

impl Pizzable for Pizza {
    fn get_price(&self) -> f64 {
        self.price
    }
}

struct PizzaWithAddon {
    name_addon: String,
    pizza: Pizza,
}

impl PizzaWithAddon {
    fn new(name: String, price: f64, name_addon: String) -> Self {
        PizzaWithAddon {
            name_addon: name_addon,
            pizza: Pizza {
                name: name,
                price: price
            }
        }
    }
}

impl Pizzable for PizzaWithAddon {
    fn get_price(&self) -> f64 {
        self.pizza.get_price() + 0.50
    }
}

struct Basket {
    pizzas: Vec<Box<Pizzable>>
}

trait Summable {
    fn get_sum(self) -> f64;
}

impl Summable for Basket {
    fn get_sum(self) -> f64 {
        let mut tot: f64 = 0.0;
        for pizza in self.pizzas {
            tot = tot + pizza.get_price();
        }
        tot
    }
}

fn print_basket(basket: Basket) {
    println!("Basket------------");
    for pizza in basket.pizzas {
        println!("");
    }
}

fn list_of_pizzas() {

}

fn main() {
    let pm = Pizza::new("Margherita".to_string(), 6.10);
    let pa = PizzaWithAddon::new("Diavola".to_string(), 8.34, "ketchup".to_string());
    let mut bask = Basket {
        pizzas: Vec::new()
    };
    bask.pizzas.push(Box::new(pm));
    bask.pizzas.push(Box::new(pa));
    println!("{}", bask.get_sum());
}
