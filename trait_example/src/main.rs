//. struct with a Vec<F> is a uniformly typed vector.
#![allow(dead_code)]
struct CallbacksV1<F: FnMut(i32)> {
    callbacks: Vec<F>,
}

pub struct Callbacks<T> {
    callbacks: Vec<Box<FnMut(T)>>,
}
trait ExeCallBacks<T>{
    fn call (&mut self, val:T);
}

impl<T> ExeCallBacks<T> for Callbacks<T>
                        where T: Copy{
    fn call(&mut self, val: T) {
        for callback in self.callbacks.iter_mut() {
            (&mut *callback)(val);
        }
    }
}

impl<T> Callbacks<T> {
    pub fn new() -> Self {
       Callbacks { callbacks: Vec::new() }
   }

   pub fn register(&mut self, callback: Box<FnMut(T)>) {
        self.callbacks.push(callback);
    }

    pub fn register_generic<F: FnMut(T)+'static>(&mut self, callback: F) {
        self.callbacks.push(Box::new(callback));
    }

    // pub fn call(&mut self, val: T) {
    //     for callback in self.callbacks.iter_mut() {
    //         (&mut *callback)(val);
    //     }
    //}
}

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

impl Foo {
    fn foo() ->(){
        println!("standalone Foo");
        }
}
pub fn main(){
    let mut c = Callbacks::new();
    c.register(Box::new(|val| println!("Callback 1 : {}", val)));
    c.call(1.20);c.call(10.909);
    {
        let mut d = Callbacks::new();
        let mut count: usize =0;
        d.register_generic(move |val|{
            count +=1;
            println!("Callback 2: {} ({}. time)", val, count);
        });
        d.call(1);d.call(2);d.call(3);
    }



    let b = Baz;
    Bar::f(&b);
    Foo::f(&b);
    <Baz as Bar>::f(&b); //Using the angle bracket syntax lets you call the trait method instead of the inherent one.
    Foo::foo();

}
