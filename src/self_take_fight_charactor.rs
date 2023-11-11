// create a thing can be created 

struct Healer <T:Heal> {
    character: T,
}
// but not declear which type it is, but I want the thing had been 
// impl the Heal trait 
// even I do not know what is it

trait Heal {
    fn heal(&mut self);
}

// now I have a Charactor struct

struct Charactor {
    name: String,
}

// and I want to impl the Heal trait for Charactor
// but if I impl the Heal for All Charactor, it does not make sense
// so I only allow who is perfessor of healing and do it 

impl<T : Heal> Heal for Healer<T>