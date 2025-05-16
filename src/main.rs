struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}
fn main() {
    let coffee = make_coffee(String::from("Late"), 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );
}
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

/*
        //Class 147
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
        };

let mut beverage = Coffee {
    name: String::from("mocha"),
    price: 4.99,
    is_hot: false,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = true;

    println!(
    "My {} this morning cost {}. It is {} that it was hot",
        beverage.name, beverage.price, beverage.is_hot
        );
*/
/*//Class 146
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    };

    let mocha = Coffee {
        name: String::from("mocha"),
        price: 4.99,
        is_hot: false,
    };
}*/
/* Class 145
println!(
    "My {} this morning cost {}. It is {} that it was hot",
    mocha.name, mocha.price, mocha.is_hot
    )*/

/*    //Class 141
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosteas Flakes"),
        String::from("Cocoa Puffs")
        ];

        let first_two = &cereals[..2];
        println!("{:?}", first_two);

        let mid_three = &cereals[1..4];
        println!("{:?}", mid_three);

        let last_three = &mut cereals[2..];
        println!("{:?}", last_three);

        last_three[1] = String::from("Lucky Charms");
        println!("{:?}", cereals);

        let cookie_crisp = &cereals[0];
        let cookie = &cookie_crisp[..6];
        println!("{:?}", cookie);

        let cocoa_puffs = &cereals[3];
        let puffs = &cocoa_puffs[6..];
        println!("{:?}", puffs);

}*/
/*
//Class 139
    let mut my_array = [10,15,20,25,30];
    let my_slyce = &mut my_array[2..4];

    my_slyce[0] = 100;
    println!("My Slice: {:?}", my_slyce);
    println!("My Array: {:?}", my_array);
*/
