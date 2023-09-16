#[derive(Debug, Clone)]
struct Base {
    value: i32,
    name: String,
}

#[derive(Debug)]
struct Foo {
    base: Base,
    something: bool,
}

#[derive(Debug)]
struct Boo {
    base: Base,
    something_else: bool,
}

trait Basable {
    fn base(&mut self) -> &mut Base;
}

impl Basable for Foo {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }
}

impl Basable for Boo {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }
}

trait Nameable {
    fn set_name(&mut self, v: String);
    fn get_name(&mut self) -> &str;
}

impl<T> Nameable for T
where
    T: Basable,
{
    fn set_name(&mut self, v: String) {
        let b = self.base();
        b.name = v;
    }
    fn get_name(&mut self) -> &str {
        self.base().name.as_str()
    }
}

fn set_name<T: Basable>(item: &mut T, name: String) {
    item.base().name = name;
}

fn get_name<T: Basable>(item: &mut T) -> &str {
    item.base().name.as_str()
}

impl From<&Foo> for Base {
    fn from(value: &Foo) -> Self {
        let b = value.base.clone();
        return b;
    }
}

fn main() {
    let mut foo = Foo {
        something: true,
        base: Base {
            value: 34,
            name: "foool".to_string(),
        },
    };

    let mut boo = Boo {
        something_else: false,
        base: Base {
            value: 23,
            name: "boooooo".to_string(),
        },
    };

    println!("foo struct:  {:?}", foo);
    println!("boo struct:  {:?}", boo);
    println!("foo base struct:  {:?}", foo.base());
    println!("boo base struct:  {:?}", boo.base());

    boo.set_name("new booo name".to_string());
    println!("boo base with new name:  {:?}", boo.base());
    println!("boo only name:  {:?}", boo.get_name());
    foo.set_name("new fooo name".to_string());
    println!("foo new name view from bas: e{:?}", foo.base());
    println!("foo only new name: {:?}", foo.get_name());

    set_name(&mut foo, "brand new name".to_string());
    println!("foo name from get_name function:  {}", get_name(&mut foo));

    let mut b: Base = (&foo).into();
    println!("From foo to Base:  {:?}", b);

    b.name = "base name changed".to_string();
    println!("From foo to Base after change name:  {:?}", b);
    println!("foo struct:  {:?}", foo);
}
