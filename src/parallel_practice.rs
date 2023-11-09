// 随堂练习
//
trait AImagineSomethingMustBehaviorWithThis {
    fn oh_my_god(&self);
}
impl<T: EvenIdoNotKnowWhatTypeButIcanTraitItSomethingLikeThis> AImagineSomethingMustBehaviorWithThis
    for T
{
    fn oh_my_god(&self) {
        println!("oh my god");
    }
}

// try
trait Printable {
    fn print(&self);
}
struct Book{}
struct Magazine{}
struct Photo{}
impl Printable for Book {
    fn print(&self) {
        println!("Book");
    }
}
impl Printable for Magazine {
    fn print(&self) {
        println!("Magazine");
    }
}
fn print_something<T: Printable>(something: T) {
    something.print();
}

trait Readable{ 
    fn read(&self);
}
impl <T: Printable>  Readable for T {
    fn read(&self) {
        println!("read");
    }
}
// 这里的实现，在说，基于printable的实现下，实现readable
// 给我的感觉是一个链条式的关系。
trait Shareable {
    fn share(&self);
}
impl <T: Readable+Printable> Shareable for T {
    fn share(&self) {
        println!("share");
    }
}
fn share_something<T: Shareable>(item: T) {
    item.share();
}
// 想到了一点，因为在ts中，这条函数可能是一样，
// 但是在share的这函数的实现，可能都需要写在每个联合的类型里，因为在ts中的组合类型是shareable的话，
// 他顶多是 Readable|Printable来使用，
// 而Shareable这个类型实则是不存在的类型，
// 所以必须在Readable和Printable的接口内也有share才行