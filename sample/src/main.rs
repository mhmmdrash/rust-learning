struct Man {
    name: String,
}

impl Man {
    fn new() -> Man {
        Man
    }
}

struct Gun;

trait Guns {
    type Uzi: Shoot;
}

trait Shoot {
    fn shoot(&self);
}

impl Shoot for Gun {
    fn shoot(&self) {
        println!("Bang Bang");
    }
}

impl Guns for Man {
    type Uzi = Gun;
}

fn main() {
    let man = Man::new();
    let uzi = <Man as Guns>::Uzi::shoot(&Gun);
    // Alternatively, you can use the instance of the associated type from the `Man` struct
    // let uzi = man::Uzi::shoot(&Gun);
    uzi;
}
