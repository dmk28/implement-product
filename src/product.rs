use std::fmt;

#[derive(Debug)]

pub struct Product {
    name: String,
    price: f32,
    qtd: i32
}

impl Product {

    pub fn new(name: String, price: f32,  qtd: i32) -> Product {
        Product {name:name,
                price: price,
                qtd: qtd,
        }
    }

    pub fn change_product_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn change_product_price(&mut self, price: f32) {
        self.price = price;
    }

    pub fn change_product_qtd(&mut self, qtd: i32) {
        self.qtd = qtd;
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {

        write!(
            f,
            "Product Name: {}\nPrice: {}\nQuantity: {}",
            self.name, self.price, self.qtd
        )

    }
}