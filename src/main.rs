pub trait System {
    fn run(&mut self);
}

pub struct App {
    systems: Vec<Box<dyn System>>,
}

impl App {
    pub fn new() -> Self {
        Self { systems: vec![] }
    }
    pub fn add_system<T: System + 'static>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run(&mut self) {
        for item in self.systems.iter_mut() {
            item.run();
        }
    }
}
impl<T: FnMut()> System for T {
    fn run(&mut self) {
        self();
    }
}
fn main() {
    println!("Hello, world!");
    App::new()
        .add_system(test_system)
        .add_system(test_system2)
        .run();
}
fn test_system() {
    println!("test_system");
}
fn test_system2() {
    println!("test_system2");
}
// 但是目前还没有带有参数的,相当于一个固定执行的system。
// 于是大家考虑把参数也加进来
// 上面是思路的由来

// impl <T: FnMut(u32)>System for T {
//     fn run(&mut self) {
//         self(323);
//     }
// }

pub trait SystemParam{}
impl SystemParam for u32 {}
impl SystemParam for bool {}
impl SystemParam for String {}

pub trait SystemParamFunction<Param>: 'static {
    fn run(&mut self, param: Param);
}
impl <F> SystemParamFunction<()> for F 
    where 
F : FnMut() + 'static, {
    fn run(&mut self, param: ()) {
        (self)();
    }
}
