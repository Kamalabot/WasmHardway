mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
    let x = MyBox(10);
    let stx = MyBox("myboax");
    let y: &i32 = &*x;
    let sty: &str = &*stx;
    println!("Deref i32 y:{}", y);
    println!("Deref &str sty:{}", sty);
    let my = MyWrapper::new("tdyn".to_owned());
    // following function prints MyWrapper type
    // without using Display trait
    print_str(my);
}

use std::ops::Deref;
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
} // Deref allows MyBox<i32> to be used as &i32 println!("Deref value: {}", y); }`

struct MyWrapper(String);

impl AsRef<str> for MyWrapper {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl MyWrapper {
    fn new(val: String) -> Self {
        MyWrapper(val)
    }
}

fn print_str(value: impl AsRef<str>) {
    println!("AsRef value: {}", value.as_ref());
}
