// 定义一个 `Action` trait，它有一个 `action` 方法
pub trait Action {
    fn action(&self);
}

// 定义一个 `Player` 结构体
pub struct Player {
    pub name: String,
}

// 为 `Player` 结构体实现 `Action` trait
impl Action for Player {
    fn action(&self) {
        println!("{} attacks!", self.name);
    }
}

// 定义一个 `Monster` 结构体
pub struct Monster {
    pub name: String,
}

// 为 `Monster` 结构体实现 `Action` trait
impl Action for Monster {
    fn action(&self) {
        println!("{} defends!", self.name);
    }
}

// 定义一个 `perform_action` 函数，这个函数接受一个实现了 `Action` trait 的对象
pub fn perform_action<T: Action>(character: T) {
    character.action();
}

fn main() {
    // 创建一个 `Player` 对象
    let my_player = Player {
        name: String::from("Hero"),
    };

    // 创建一个 `Monster` 对象
    let my_monster = Monster {
        name: String::from("Dragon"),
    };

    // 调用 `perform_action` 函数，传入 `my_player` 对象
    perform_action(my_player);

    // 调用 `perform_action` 函数，传入 `my_monster` 对象
    perform_action(my_monster);
}