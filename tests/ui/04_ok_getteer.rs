use rumbok::Getter;

#[derive(Getter)]
struct User<'a> {
    id: i32,
    name: String,
    value: &'a str,
}

fn main() {
    let u = User {
        id: 1,
        name: String::from("a"),
        value: "test",
    };
    let _id = u.get_id();
    let _name = u.get_name();
    let _value = u.get_value();
}
