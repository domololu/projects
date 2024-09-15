fn main() {
    let mut dive: Submarine = create_submarine(String::from("Dive"));

    {
        let _x: &String = &dive.name;
        dive.weight;
        dive.length;
        dive.height;
        dive.minimum_depth;

        dive.name();
        dive.weight();
        dive.length();
        dive.height();
        dive.minimum_depth();
    }

    // dive.weight() = 5;

    dive.change_name(String::from("Glide"));
    dive.change_weight(5);
    dive.change_length(6);
    dive.change_height(7);
    dive.change_minimum_depth(8);

    // dive.weight = 5;

    println!(
        "The properties' values have been redefined. The new values are:
        name: {},
        weight: {},
        length: {},
        height: {},
        minimum_depth: {}",
             dive.name(), dive.weight(), dive.length(), dive.height(), dive.minimum_depth()
    );
}


struct Submarine {
    name: String,
    weight: i32,
    length: i32,
    height: i32,
    minimum_depth: i32,
}



impl Submarine {

    fn name(&self) -> &String {
        &self.name
    }

    /* fn change<T>(&mut self, _a:T, _b:&str) {
        // for another time!
    }
    */

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn weight(&self) -> i32 {
        self.weight
    }

    fn change_weight(&mut self, a: i32) {
        self.weight = a;
    }

    fn length(&self) -> i32 {
        self.length
    }

    fn change_length(&mut self, a: i32) {
        self.length = a;
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn change_height(&mut self, a: i32) {
        self.height = a;
    }

    fn minimum_depth(&self) -> i32 {
        self.minimum_depth
    }

    fn change_minimum_depth(&mut self, a: i32) {
        self.minimum_depth = a;
    }
}



fn create_submarine(name: String) -> Submarine {
    println!(
        "The properties of the {name} submarine and their values are:
        name: {name},
        weight: 0,
        length: 0,
        height: 0,
        minimum_depth:0,
        "
    );

    let sub: Submarine = Submarine {
        name,
        weight: 0,
        length: 0,
        height: 0,
        minimum_depth: 0,
    };

    sub
}
