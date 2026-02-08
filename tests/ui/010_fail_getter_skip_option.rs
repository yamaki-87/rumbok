use rumbok::Getter;

#[derive(Getter)]
struct User<'a> {
    id: i32,
    name: String,
    value: &'a str,
    #[getter(skip)]
    nil: Option<i32>,
}

fn main() {
    let u = User {
        id: 1,
        name: String::from("a"),
        value: "test",
        nil: None,
    };
    let _id = u.get_id();
    let _name = u.get_name();
    let _value = u.get_value();
    // ここでコンパイルエラーを期待
    let _nil = u.get_nil();
}
