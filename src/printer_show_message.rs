
// 定义一个 `Printable` trait
pub trait Printable {
    fn print(&self);
}

// 定义一个 `Show` 结构体
pub struct Show<T: Printable>(T);

// 为 `Show` 结构体实现 `Printable` trait
impl<T: Printable> Printable for Show<T> {
    fn print(&self) {
        self.0.print();
        println!("Show!");
    }
}

// 定义一个 `Message` 结构体
pub struct Message {
    pub content: String,
}

// 为 `Message` 结构体实现 `Printable` trait
impl Printable for Message {
    fn print(&self) {
        println!("{}", self.content);
    }
}

fn main() {
    // 创建一个 `Message` 对象
    let my_message = Message {
        content: String::from("Hello, world!"),
    };

    // 创建一个 `Show` 对象，它包装了 `my_message` 对象
    let show_message = Show(my_message);

    // 调用 `print` 方法
    show_message.print();
}