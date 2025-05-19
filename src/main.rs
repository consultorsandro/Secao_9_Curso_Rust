#[derive(Debug)] // Convertendo a struct para um formato legível/imprimir
struct TaylorSwift {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwift {
    // Immutable struct value (self parameter takes ownership)
    // Mutable struct value (self parameter takes ownership, has permission to change)
    // Immutable reference to struct instance (no ownership)
    // Mutable reference to struct instance (no ownership, has permission to change)
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds ", self.duration_secs);
    }

    fn double_length(mut self: Self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self);
    }
    // Class 156
    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }
    
}

fn main() {

//let song = TaylorSwift {
    let blank_space = TaylorSwift {
    title: String::from("Blank Space"),
    release_year: 2014,
    duration_secs: 231,
    };
   // song.display_song_info(); // class 153
   // song.double_length(); // class 154
   let all_too_well = TaylorSwift {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!("{} is longer than {}", blank_space.title, all_too_well.title);
    
    } else {
        println!("{} is shoter than or equal to {}", blank_space.title, all_too_well.title);
    }


    
}

/*
    //Class 152
    let mut mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
    println!("{:?}", mocha); //{:?} é o mesmo que {:#?} mas sem o espaçamento
    println!("{:#?}", mocha);
*/
/*
//Class 152 outside of main
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    //Podemos mudar o valor de coffee
    // não é necessário (*coffee).name
    println!("Drinking my delicious {}", coffee.name);
}
*/

/*
fn main() { //Class 149
    let name = String::from("Latte");
   let coffee = make_coffee(name, 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );
    let name = String::from("Latte"); //Class 149
    let price = 3.99;
    let is_hot = false;
    let latte = Coffee {
        name: name.clone(),
        price,
        is_hot,
    };

}
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
*/
/*
    //Class 148
    let coffee = make_coffee(String::from("Late"), 4.99, true);
    println!(
        "My {} this morning cost {}. It is {} that it was hot.",
        coffee.name, coffee.price, coffee.is_hot
    );
}
//Class 148 outside of main
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

*/
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
