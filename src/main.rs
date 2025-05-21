use std::fmt::DebugSet;

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}
impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }
    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination
    }
    
    fn increase_price(&mut self) {
        self.price *= 1.20;
    }
    fn itinerary(&self) {
        println!("Flight from {} to {}", self.origin, self.destination);
    }
}

fn main() {
//Class 164 Project
let mut my_flight = Flight::new(
    String::from("New York"),
    String::from("Los Angeles"),
    299.99,150,
);
println!("{:?}", my_flight);

my_flight.change_destination(String::from("San Francisco"));
my_flight.increase_price();
my_flight.itinerary();
println!("{:?}", my_flight);

let another_flight = Flight{

    origin: String::from("Paris"),
    destination: String::from("Rome"),
    ..my_flight
};
println!("{:#?}", another_flight);

}
 
/* outside of main
//Hours, minutes
struct ShorDuration(u32, u32);
//Years, months
struct LongDuration(u32, u32);

fn go_to_work(lenght: ShorDuration) {
    println!("Passing time {} hours {} minutes", lenght.0, lenght.1);
}

fn accept_tuple(lenght: (u32, u32)) {}
    //Class 161
    let worh_shift = ShorDuration(8, 0);
    println!("{} hours {} minutes", worh_shift.0, worh_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);
*/
/*
    let mut computer = Computer::new(String::from("M3 Max"), 64, 8);
    // Preciso consertar isso
    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(4);
    /*
    .upgrade_cpu(String::from("M4 Max"))
    .upgrade_memory(128)
    .upgrade_hard_drive_capacity(4);
     */

    println!("Stats {:#?}", computer)
}
 Class 160 outside of main
    //Class 160
    #[derive(Debug)]
struct Computer { //Class 160 The Build Pattern
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
    }

    impl Computer {
        fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
            Self {
                cpu,
                memory,
                hard_drive_capacity,
            }
        }

        fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self{
            self.cpu = new_cpu;
            self
        }

        fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
            self.memory = new_memory;
            self
        }
        fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
            self.hard_drive_capacity = new_capacity;
            self
        }
    }
*/
/*
    //let song = TaylorSwift {
    // Class 158
    let blank_space = TaylorSwift::new(String::from("Blank Space"), 2014, 231);

    /* let blank_space = TaylorSwift {
    title: String::from("Blank Space"),
    release_year: 2014,
    duration_secs: 231,
    };*/
    // song.display_song_info(); // class 153
    // song.double_length(); // class 154
    let all_too_well = TaylorSwift {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is shoter than or equal to {}",
            blank_space.title, all_too_well.title
        );
    }

    blank_space.display_song_info(); // class 157
}
*/
/* // class 168 outside of main
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

    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {  // Class 158
        Self {
            // Self is a type alias for the struct, in capital letters
            title,
            release_year,
            duration_secs,
        }
    }

    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Year since release: {}", self.years_since_release());
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

    fn years_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

*/
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
