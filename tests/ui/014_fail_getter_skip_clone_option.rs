use rumbok::Getter;

#[derive(Getter)]
struct Wrapper<'a, T>
where
    T: Clone,
{
    value: T,
    id: i32,
    name: String,
    refer: &'a str,
    #[getter(clone, skip)]
    err: Result<i32, String>,
}
fn main() {
    let u = Wrapper {
        value: String::from("a"),
        id: 2,
        name: "test".into(),
        refer: "&str",
        err: Ok(1),
    };

    assert_eq!("a", u.get_value());
    assert_eq!(2, *u.get_id());
    assert_eq!("test", u.get_name());
    assert_eq!("&str", u.get_refer());
    let expected_err: Result<i32, String> = Err(String::from("err"));
    assert_eq!(expected_err, u.get_err());
}
