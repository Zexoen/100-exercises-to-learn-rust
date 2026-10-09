// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(p: String,q: i32,u: i32) -> Self {
        if p.len() == 0 || p.len() > 300 {
            panic!()
        }
        if q <= 0 || u ==0 {
            panic!();
        }
        Self {
            product_name : p,
            quantity : q,
            unit_price : u,
        }
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
    pub fn set_product_name(&mut self,p: String) {
        self.product_name = p;
    }
    pub fn set_quantity(&mut self,q: i32) {
        self.quantity = q;
    }
    pub fn set_unit_price(&mut self,u: i32) {
        self.unit_price = u;
    }
    pub fn total(&self) -> i32 {
        self.unit_price * self.quantity
    }
}
