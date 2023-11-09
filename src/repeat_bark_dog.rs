// 定义一个 `Speak` trait
pub trait Speak {
    fn speak(&self);
}

// 定义一个 `Repeat` 结构体
pub struct Repeat<T: Speak>(T);

// 为 `Repeat` 结构体实现 `Speak` trait
impl<T: Speak> Speak for Repeat<T> {
    fn speak(&self) {
        self.0.speak();
        self.0.speak();
    }
}

// 定义一个 `Dog` 结构体
pub struct Dog {
    pub name: String,
}

// 为 `Dog` 结构体实现 `Speak` trait
impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

fn main() {
    // 创建一个 `Dog` 对象
    let my_dog = Dog {
        name: String::from("Fido"),
    };

    // 创建一个 `Repeat` 对象，它包装了 `my_dog` 对象
    let repeat_dog = Repeat(my_dog);

    // 调用 `speak` 方法
    repeat_dog.speak();
}