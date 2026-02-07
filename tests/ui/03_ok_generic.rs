use rumbok::Setter;

#[derive(Setter)]
struct Wrapper<T>
where
    T: Clone,
{
    value: T,
}
fn main() {
    let mut u = Wrapper { value: "a".into() };
    u.set_value("b");
}
