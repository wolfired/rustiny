trait Foo {
    fn foo(self);
}

trait Bar {
    fn bar(&self);
}

struct Sun;

// impl Foo for Sun {
//     fn foo(self) {
//         println!("foo: v");
//     }
// }

impl Foo for &Sun {
    fn foo(self) {
        println!("foo: r");
    }
}

impl Bar for Sun {
    fn bar(&self) {
        println!("bar: r");
    }
}

fn trace_foo<T>(t: T)
where
    for<'a> &'a T: Foo,
{
    t.foo();
}

fn trace_bar<T>(t: T)
where
    T: Bar,
{
    t.bar();
}

fn main() {
    let sf = Sun;
    // sf.foo();

    trace_foo(sf);

    let sb = Sun;
    // sb.bar();

    trace_bar(sb);

    println!("Hello, rustiny world!");
}
