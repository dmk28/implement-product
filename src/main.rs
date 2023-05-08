mod product;
use product::Product;
fn main() {
    
    let product_name = "Zephyrus G14";
    let product_price = 1500.00;
    let qtd = 10;

    let mut product = Product::new(product_name.to_string(), product_price, qtd);
    println!("{}\n", product);
    let new_name = "Dell G15";
    product.change_product_name(new_name.to_string());
    product.change_product_qtd(5);
    product.change_product_price(500.00);
    println!("{}", product);






}
