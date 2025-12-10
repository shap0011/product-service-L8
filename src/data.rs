use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
    Product {
        id: 1,
        name: "Samsung 55\" 4K UHD Smart TV".to_string(),
        price: 599.99,
        description: "A stunning 4K Ultra HD television with vivid colors, HDR support, and built-in streaming apps.".to_string(),
        image: "/tv.jpg".to_string(),
    },
    Product {
        id: 2,
        name: "Sony Noise Cancelling Headphones".to_string(),
        price: 349.99,
        description: "Premium wireless headphones with industry-leading noise cancellation and 30-hour battery life.".to_string(),
        image: "/headphones.jpg".to_string(),
    },
    Product {
        id: 3,
        name: "Apple iPad (10th Gen) 64GB".to_string(),
        price: 449.99,
        description: "Sleek and powerful tablet featuring a Liquid Retina display and the A14 Bionic chip.".to_string(),
        image: "/ipad.jpg".to_string(),
    },
    Product {
        id: 4,
        name: "Dell Inspiron 15.6\" Laptop".to_string(),
        price: 799.99,
        description: "Everyday performance laptop with Intel Core i5, 16GB RAM, and 512GB SSD storage.".to_string(),
        image: "/laptop.jpg".to_string(),
    },
    Product {
        id: 5,
        name: "Logitech Wireless Keyboard + Mouse Combo".to_string(),
        price: 59.99,
        description: "Comfortable, quiet typing with smooth mouse tracking; ideal for office and home use.".to_string(),
        image: "/keyboard.jpg".to_string(),
    },
    Product {
        id: 6,
        name: "Nintendo Switch OLED Console".to_string(),
        price: 449.99,
        description: "Popular hybrid gaming console with stunning OLED display.".to_string(),
        image: "/switch.jpg".to_string(),
    },
    Product {
        id: 7,
        name: "Canon EOS M50 Mark II Camera".to_string(),
        price: 999.99,
        description: "Mirrorless camera perfect for photography enthusiasts and content creators.".to_string(),
        image: "/camera.jpg".to_string(),
    },
    Product {
        id: 8,
        name: "Fitbit Charge 6 Fitness Tracker".to_string(),
        price: 229.99,
        description: "Track your activity, heart rate, sleep, and health metrics with all-day battery life.".to_string(),
        image: "/fitbit.jpg".to_string(),
    },
    Product {
        id: 9,
        name: "Bose Bluetooth Home Speaker".to_string(),
        price: 299.99,
        description: "Rich, immersive audio with voice assistant support and Wi-Fi streaming.".to_string(),
        image: "/speaker.jpg".to_string(),
    },
    Product {
        id: 10,
        name: "HP Envy Wireless All-in-One Printer".to_string(),
        price: 199.99,
        description: "Compact home office printer with wireless printing and scanning capabilities.".to_string(),
        image: "/printer.jpg".to_string(),
    },
]

}