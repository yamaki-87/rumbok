use rumbok::Getter;

#[derive(Getter)]
struct Wrapper<T>
where
    T: Clone,
{
    value: T,
}
fn main() {
    let u = Wrapper {
        value: String::from("test"),
    };
    let _value = u.get_value();
}
