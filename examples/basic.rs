extern crate corsair_keyb_led as led;

fn main() {
    let driver = led::Driver::init(false);

    println!("{:?}", driver);
}
