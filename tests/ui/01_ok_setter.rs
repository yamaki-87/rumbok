use rumbok::Setter;

#[derive(Setter)]
struct User<'a> {
    id: i32,
    name: String,
    value: &'a str,
}

fn main() {
    let mut u = User {
        id: 1,
        name: String::from("a"),
        value: "test",
    };
    u.set_id(10);
    u.set_name(String::from("b"));
    u.set_value("test_ok");
}
