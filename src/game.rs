use std::collections::HashMap;

static mut WIDTH: u32 = 100;
static mut HEIGHT: u32 = 100;
pub struct Game {
    field: Vec<Vec<char>>, //the map-characters (used for drawing to the screen)
    //As the game progresses, the following lists completely demolish the FPS
    plants: HashMap<logic::Coordinate, logic::Plant>, //plants mapped to coordinates. note that every coordinate has a plant object, they just count how many there are on that spot
    animals: Vec<logic::Animal>, //every alive animal
}

mod logic {
    use std::hash::Hasher;
    pub struct Plant {
        amount: u32,
    }

    pub struct Animal {
        x: u32,
        y: u32,
        energy: u32,
        genes: [u8; 8], //genes decide the probability of every direction (8 directions => 8 genes)
        direction: usize //0 = top left, 1 = top, 2 = top right, 3 = right, etc.
    }

    pub struct Coordinate {
        x: u32,
        y: u32
    }

    impl Plant {
        pub fn new() -> Plant {
            Plant {
                amount: 0
            }
        }

        pub fn get_amount(&mut self) -> u32 {
            self.amount
        }

        pub fn decrease(&mut self) {
            if self.amount > 0 {
                self.amount = self.amount-1;
            }
        }

        pub fn increase(&mut self) {
            self.amount = self.amount+1;
        }
    }

    impl Animal {
        /**
         * Create a new animal with random genes and positioned in the middle.
         */
        pub fn new(x: u32, y: u32, energy: u32) -> Animal {
            let mut a: Animal = Animal {
                x: x,
                y: y,
                energy: energy,
                genes: [0; 8],
                direction: 0
            };
            use crate::rand::Rng;
            for i in 0..8 {
                
                a.genes[i] = rand::thread_rng().gen_range(1, 11);
            }
            a.direction = a.determine_direction();
            return a;
        }

        /**
         * Create a new animal which inherits and mutates the genes of the parent.
         * It also keeps the position of the parent
         */
        fn new_child(x: u32, y: u32, energy: u32, genes: &[u8; 8]) -> Animal {
            let mut a: Animal = Animal {
                x: x,
                y: y,
                energy: energy,
                genes: [0; 8],
                direction: 0
            };
            for i in 0..8 {
                
                a.genes[i] = genes[i];
            }
            a.mutate();
            a.direction = a.determine_direction();
            return a;
        }

        /**
         * Choose one of the 8 genes and increase or decrease it by one.
         * Every gene has to be a number between 1 and 10
         */
        fn mutate(&mut self) {
            use rand::Rng;
            let mut rng: rand::ThreadRng = rand::thread_rng();
            let ind: usize = rng.gen_range(0, 8);
            if self.genes[ind] > 1 && self.genes[ind] < 10{
                if rng.gen() {
                    self.genes[ind] = self.genes[ind]+1;
                }else {
                    self.genes[ind] = self.genes[ind]-1;
                }
            }else if self.genes[ind] == 1 {
                self.genes[ind] = self.genes[ind]+1;
            }else if self.genes[ind] == 10 {
                self.genes[ind] = self.genes[ind]-1;
            }
        }

        /**
         * If the animal has enough energy, half its energy and give it
         * to the newborn. Otherwise don't create a new animal.
         */
        pub unsafe fn reproduce(&mut self) -> Option<Animal> {
            if self.energy > ENERGY_TO_REPRODUCE {
                let split: u32 = self.energy / 2;
                self.energy = split;
                return Option::from(Animal::new_child(
                    self.x,
                    self.y,
                    split,
                    &self.genes
                ));
            }
            return None;
        }

        /**
         * Decrease energy by one, move, pick a new direction for the next day.
         * Returns false if the animal died. (Energy below 0)
         */
        pub fn new_day(&mut self) -> bool {
            if self.energy < 1 {
                return false;
            }
            self.energy = self.energy-1;
            self.move_animal();
            self.direction = self.determine_direction();
            return true;
        }

        pub fn eat(&mut self, energy: u32) {
            self.energy = self.energy+energy;
        }

        pub fn get_x(&mut self) -> u32 {
            self.x
        }

        pub fn get_y(&mut self) -> u32{
            self.y
        }

        /**
         * Algorithm for selecting the new direction to take.
         */
        fn determine_direction(&mut self) -> usize {
            use crate::rand::Rng;
            let mut size: usize = 0;
            for e in &self.genes {
                size = size+(*e as usize);
            }
            let mut range: Vec<usize> = vec![0; size];
            let mut index: usize = 0;
            let mut ind: usize = 0;
            for e in &self.genes {
                for _i in 0..(*e) {
                    range[ind] = index;
                    ind = ind+1;
                }
                index = index+1;
            }
            let r_num: usize = rand::thread_rng().gen_range(0, size);
            return range[r_num];
        }

        /**
         * Move in the current direction and fix the position if the animal is out of bounds.
         */
        fn move_animal(&mut self) {
            let width: u32;
            let height: u32;
            unsafe {
                width = super::WIDTH;
                height = super::HEIGHT;
            }
            match self.direction {
                0 => {
                    if self.x > 0 {
                        self.x = self.x-1;
                    }else {
                        self.x = width-1;
                    }
                    
                    if self.y > 0 {
                        self.y = self.y-1;
                    }else {
                        self.y = height-1;
                    }
                },
                1 => {
                    if self.y > 0 {
                        self.y = self.y-1;
                    }else {
                        self.y = height-1;
                    }
                },
                2 => {
                    self.x = self.x+1;
                    if self.y > 0 {
                        self.y = self.y-1;
                    }else {
                        self.y = height-1;
                    }
                },
                3 => {
                    self.x = self.x+1;
                },
                4 => {
                    self.x = self.x+1;
                    self.y = self.y+1;
                },
                5 => {
                    self.y = self.y+1;
                },
                6 => {
                    if self.x > 0 {
                        self.x = self.x-1;
                    }else {
                        self.x = width-1;
                    }
                    self.y = self.y+1;
                },
                _ => {
                    if self.x > 0 {
                        self.x = self.x-1;
                    }else {
                        self.x = width-1;
                    }
                }
            }
            self.fix_position();
        }

        fn fix_position(&mut self) {
            let width: u32;
            let height: u32;
            unsafe {
                width = super::WIDTH;
                height = super::HEIGHT;
            }
            if self.x >= width {
                self.x = self.x % width;
            }
            if self.y >= height {
                self.y = self.y % height;
            }
        }
    }

    impl Coordinate {
        pub fn new(x: u32, y: u32) -> Coordinate {
            Coordinate {
                x: x,
                y: y
            }
        }

        pub fn get_x(&self) -> u32 {
            self.x
        }

        pub fn get_y(&self) -> u32 {
            self.y
        }
    }

    impl std::cmp::PartialEq for Coordinate {
        fn eq(&self, other: &Coordinate) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl std::hash::Hash for Coordinate {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.x.hash(state);
            self.y.hash(state);
        }
    }

    impl std::cmp::Eq for Coordinate {}

    pub static mut PLANT_ENERGY: u32 = 60;
    pub static mut ENERGY_TO_REPRODUCE: u32 = 60;
    pub static mut PLANTS_PER_DAY: u32 = 1;
    pub static mut PLANTS_PER_DAY_FOREST: u32 = 1;
    pub static mut FOREST_PERCENT: f32 = 0.1;
}



impl Game {
    /**
     * Create a new simulation with the given settings
     */
    pub fn new(width: u32, height: u32, plant_energy: u32, energy_to_reproduce: u32, plants_per_day: u32, plants_per_day_forest: u32, forest_percent: f32) -> Game {
        let mut s = Game {
            field: Vec::new(),
            plants: HashMap::new(),
            animals: Vec::new()
        };
        unsafe {
            WIDTH = width;
            HEIGHT = height;
            logic::PLANT_ENERGY = plant_energy;
            logic::ENERGY_TO_REPRODUCE = energy_to_reproduce;
            logic::PLANTS_PER_DAY = plants_per_day;
            logic::PLANTS_PER_DAY_FOREST = plants_per_day_forest;
            logic::FOREST_PERCENT = forest_percent;
        }
        let mut index: u32 = 0;
        //Initialize map-chars and plants
        while index < height {
            let mut index2: u32 = 0;
            let mut v: Vec<char> = Vec::new();
            while index2 < width {
                v.push(' ');
                s.plants.insert(logic::Coordinate::new(index2, index), logic::Plant::new());
                index2 = index2+1;
            }
            s.field.push(v);
            index = index+1;
        }
        //First animal
        s.animals.push(logic::Animal::new((width/2) as u32, (height/2) as u32, 1000));
        return s;
    }

    pub fn get_width(&mut self) -> u32 {
        let width: u32;
        unsafe {
            width = WIDTH;
        }
        return width;
    }

    pub fn get_height(&mut self) -> u32 {
        let height: u32;
        unsafe {
            height = HEIGHT;
        }
        return height;
    }

    /**
     * Set a char in the map-chars.
     */
    pub fn set_single(&mut self, x: usize, y: usize, c: char) {
        self.field[y][x] = c;
    }

    /**
     * Get a char from the map-chars
     */
    pub fn get_single(&mut self, x: usize, y: usize) -> char {
        self.field[y][x]
    }

    /**
     * Start the next day.
     * -> Generate plants
     * -> Update animals (move, eat, dispose of the dead ones, etc.)
     */
    pub fn new_day(&mut self) {
        self.generate_plants();
        self.update_animals();
    }

    /**
     * Generate global and forest plants.
     */
    fn generate_plants(&mut self) {
        use rand::Rng;
        let mut rng: rand::ThreadRng = rand::thread_rng();
        let num_plants: u32;
        let num_plants_forest: u32;
        let width_forest: u32;
        let height_forest: u32;
        let width: u32;
        let height: u32;
        unsafe {
            num_plants = logic::PLANTS_PER_DAY;
            num_plants_forest = logic::PLANTS_PER_DAY_FOREST;
            width = WIDTH;
            height = HEIGHT;
            width_forest = ((width as f32) * logic::FOREST_PERCENT) as u32;
            height_forest = ((height as f32) * logic::FOREST_PERCENT) as u32;
        }
        for _i in 0..num_plants {
            let x: u32 = rng.gen_range(0, width);
            let y: u32 = rng.gen_range(0, height);
            let plant = self.plants.get_mut(&logic::Coordinate::new(x, y));
            match plant {
                Some(p) => {
                    p.increase();
                    if p.get_amount() == 1 {
                        self.set_single(x as usize, y as usize, 'P');
                    }
                },
                None => {
                    println!("ERROR: Tried to access non-existend plant: {}|{}", x, y);
                }
            }
        }
        for _i in 0..num_plants_forest {
            let x: u32 = rng.gen_range(0, width_forest) + (((width as f32 / 2.0) as u32) - ((width_forest/2) as u32));
            let y: u32 = rng.gen_range(0, height_forest) + (((height as f32 / 2.0) as u32) - ((height_forest/2) as u32));
            let plant = self.plants.get_mut(&logic::Coordinate::new(x, y));
            match plant {
                Some(p) => {
                    p.increase();
                    if p.get_amount() == 1 {
                        self.set_single(x as usize, y as usize, 'P');
                    }
                },
                None => {
                    println!("ERROR: Tried to access non-existend plant: {}|{}", x, y);
                }
            }
        }
    }

    /**
     * Update animals.
     * -> clear animals from map-chars
     * -> animal.new_day()
     * -> if it died, skip to next animal
     * -> eat plant if it stands on one
     * -> try to reproduce
     * -> dispose of dead animals
     * -> add new animals to the animals vector
     */
    fn update_animals(&mut self) {
        let mut garbage: Vec<usize> = Vec::new();
        let mut newborn: Vec<logic::Animal> = Vec::new();
        // First of all clear the animals from the screen
        let mut to_clear: Vec<logic::Coordinate> = Vec::new();
        for animal in &mut self.animals {
            to_clear.push(logic::Coordinate::new(animal.get_x(), animal.get_y()));
        }
        for coord in &to_clear {
            self.set_single(coord.get_x() as usize, coord.get_y() as usize, ' ');
            let plant = self.plants.get_mut(&coord);
            match plant {
                Some(p) => {
                    if p.get_amount() > 0 {
                        self.set_single(coord.get_x() as usize, coord.get_y() as usize, 'P');
                    }
                },
                None => {
                    println!("ERROR: Tried to access non-existend plant: {}|{}", coord.get_x(), coord.get_y());
                }
            }
        }
        to_clear.clear();

        let mut size: usize = self.animals.len();
        for i in 0..size {
            let mut e: u32 = 0;
            if let Some(anim) = self.animals.get_mut(i) {
                let x = anim.get_x();
                let y = anim.get_y();
                e = self.eat_plant(x, y);
            }
            if let Some(anim) = self.animals.get_mut(i) {
                if !anim.new_day() {
                    garbage.push(i);
                    continue;
                }
                anim.eat(e);
                unsafe {
                    if let Some(child) = anim.reproduce() {
                        newborn.push(child);
                    }
                }
            }
        }
        self.animals.append(&mut newborn);
        size = garbage.len();
        for i in (0..size).rev() {
            self.animals.remove(garbage[i]);
        }
        garbage.clear();

        for animal in &mut self.animals {
            to_clear.push(logic::Coordinate::new(animal.get_x(), animal.get_y()));
        }
        for coord in &to_clear {
            self.set_single(coord.get_x() as usize, coord.get_y() as usize, 'A');
        }
        to_clear.clear();
    }

    fn eat_plant(&mut self, x: u32, y: u32) -> u32{
        let coord: logic::Coordinate = logic::Coordinate::new(x, y);
        if let Some(plant) = self.plants.get_mut(&coord) {
            if plant.get_amount() > 0 {
                plant.decrease();
                if plant.get_amount() == 0 {
                    self.set_single(x as usize, y as usize, ' ');
                }
                unsafe {
                    return logic::PLANT_ENERGY;
                }
            }
        }
        return 0;
    }
}