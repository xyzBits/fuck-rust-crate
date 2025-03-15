/// 特征对象，通过特征来统一表示某一组类型的方式，这些类型都实现了同一个特征
/// &dyn Trait
/// Box<dyn Trait>
/// 虚表：记录了特征类型实现特征方法的地址，通过特征对象调用方法时，
/// rust会查找虚表来决定调用哪个具体的实现
trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
#[test]
fn test_01() {
    // 使用 Box<dyn Trait> 存储不同类型的对象
    // Vec<Box<dyn Animal>> 存储了 Dog Cat 的实例，它们都被视为 dyn Animal 特征对象
    // 通过动态分发，程序在运行时根据实际类型调用相应的 speak 方法
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals {
        animal.speak(); // 运行时决定调用 Dog::speak还是Cat::speak
    }
}

/// 哪些场景下使用特征对象
/// 存储异构集合，当你需要一个容器，存储不同类型的对象，且这些对象都实现了某个共同的特征
/// 插件系统：允许在运行时加载实现了某个特征的外部模块
/// 抽象接口：例如gui框架中，不同控件实现相同的绘制接口
trait Drawable {
    fn draw(&self);
}

struct Button;

impl Drawable for Button {
    fn draw(&self) {
        println!("Drawing a button");
    }
}

struct TextBox;
impl Drawable for TextBox {
    fn draw(&self) {
        println!("Drawing a textbox");
    }
}

// 使用特征对象渲染控件
fn render(components: Vec<Box<dyn Drawable>>) {
    for component in components {
        component.draw();
    }
}

#[test]
fn test_02() {
    let button = Button;
    let textbox = TextBox;
    let components: Vec<Box<dyn Drawable>> = vec![Box::new(button), Box::new(textbox)];
    render(components);
}

fn render1(components: Vec<&dyn Drawable>) {
    for component in components {
        component.draw();
    }
}

#[test]
fn test_03() {
    let button = Button;
    let textbox = TextBox;
    let components: Vec<&dyn Drawable> = vec![&button, &textbox];
    render1(components);
}

trait NotObjectSafe {
    fn method<T>(&self, t: T); // 包含泛型参数，非对象安全
}

struct Foo;
impl NotObjectSafe for Foo {
    fn method<T>(&self, _t: T) {
        // 方法中有泛型参数，无法作为特征对象的接口
        println!("Generic method");
    }
}
#[test]
fn test_04() {
    // let obf: Box<dyn NotObjectSafe> = Box::new(Foo);
}
