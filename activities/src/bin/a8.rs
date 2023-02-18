// generate a random drink with flavor and size, given some flavors and sizes of your choosing
use rand::Rng;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
enum DrinkFlavors {
    Cherry,
    Zero,
    Regular,
    Diet,
    CherryZero,
}
#[derive(Debug, EnumCountMacro, EnumIter)]
enum Sizes {
    Size1,
    Size2,
    Size3,
    Size4,
}
// function enum with constant string values
impl Sizes {
    fn as_str(&self) -> &'static str {
        match self {
            Sizes::Size1 => "330ml",
            Sizes::Size2 => "500ml",
            Sizes::Size3 => "1.75l",
            Sizes::Size4 => "2l",
        }
    }
}

#[derive(Debug)]
struct Drink {
    flavor: DrinkFlavors,
    size: Sizes,
}
// function to display flavor and size
impl Drink {
    fn display_drink(&self) {
        println!("{:?} Cola, {:?}", &self.flavor, &self.size.as_str());
    }
}

fn generate_drink(random_flavor: usize, random_size: usize) -> Drink {
    let flavor: DrinkFlavors = match random_flavor {
        1 => DrinkFlavors::Cherry,
        2 => DrinkFlavors::CherryZero,
        3 => DrinkFlavors::Diet,
        4 => DrinkFlavors::Regular,
        _ => DrinkFlavors::Zero,
    };
    let size: Sizes = match random_size {
        1 => Sizes::Size1,
        2 => Sizes::Size2,
        3 => Sizes::Size3,
        _ => Sizes::Size4,
    };
    Drink { flavor, size }
}

fn main() {
    let sizes_len = Sizes::COUNT;
    let drink_flavors_len = DrinkFlavors::COUNT;
    let today_flavor = rand::thread_rng().gen_range(1..drink_flavors_len + 1);
    let today_size = rand::thread_rng().gen_range(1..sizes_len);
    let todays_drink = generate_drink(today_flavor, today_size);
    todays_drink.display_drink();
}
