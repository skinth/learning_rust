//the file itself acts as a module: no need to mod declaration!
pub struct Pizza {
    pub name: String,
    pub price: f64
}
trait Calculable {
    fn get_price(&self) -> f64;
}
impl Calculable for Pizza {
    fn get_price(&self) -> f64 {
        self.price
    }
}
pub struct PizzaWithAddon {
    name: String,
    pub pizza: Pizza,
}
impl Calculable for PizzaWithAddon {
    fn get_price(&self) -> f64 {
        self.pizza.get_price() + 0.50
    }
}
pub enum PizzasTypes {
    Pizza,
    Diavola
}

fn main() {
    let p = Pizza {
        name: "Diavola".to_string(),
        price: 8.34
    };
    println!("{}",p.get_price());
    let pa = PizzaWithAddon {
        name: "mayonnese".to_string(),
        pizza: p
    };
    println!("{}", pa.get_price());
}
