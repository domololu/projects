use num_format::{Locale, ToFormattedString};

fn main () {
    enum PropertyType {
        Residential,
        Commercial,
        Industrial,
        Land,
    }
    enum Rooms {
        Bedroom(i32),
        Bathroom(i32),
        ConferenceRoom(i32),
        Office(i32),

    }
    enum Size {
        Acres(i32),
        SquareFeet(i32),
    }
    enum Location {
        City(String),
        Town(String),
        Unincorporated(String),
        ClosestTo(String)
    }
    struct Property<const T: usize, const Y:usize> {
        property_type: PropertyType,
        room: [Rooms; T],
        size: i32,
        address: [Location; Y],
        // price: i32,
    }
    enum Price {
        USD(i32),
        PoundSterling(i32),
        Bitcoin(i32),
        DogeCoin(i32),
    }

    let bungalow = Property {
        property_type: PropertyType::Residential,
        room: [Rooms::Bedroom(5), Rooms::Bathroom(3)],
        size: 130680,
        address: [Location::City(String::from("Big Sur"))],
    };

    fn get_property_price <const T: usize, const Y:usize> (property: Property <T, Y> ) -> String {
        let price_per_square_foot = 200;
        let property_price = price_per_square_foot * property.size;

        property_price.to_formatted_string(&Locale::en)
    }

    let a:String = get_property_price(bungalow);
    println!("{a}")
}
