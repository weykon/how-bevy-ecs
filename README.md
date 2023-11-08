# how bevy ecs

```rust
impl<T: FnMut()> System for T{
    fn run(&mut self){
        self();
    }
}
```
解读： 有一个T我不清楚的，但是我至少清楚他是需要有System的trait实现。

