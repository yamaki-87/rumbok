use rumbok::Data;

#[derive(Data)]
struct Wrapper<'a, T>
where
    T: Clone,
{
    value: T,
    id: i32,
    name: String,
    refer: &'a str,
    err: Result<i32, String>,
}
fn main() {
    let mut u = Wrapper {
        value: String::from("a"),
        id: 2,
        name: "test".into(),
        refer: "&str",
        err: Ok(1),
    };
    u.set_value("b".to_string());
    u.set_id(-32);
    u.set_name("set_name".into());
    u.set_refer("refer");
    u.set_err(Err("err".into()));

    assert_eq!("b", u.get_value());
    assert_eq!(-32, *u.get_id());
    assert_eq!("set_name", u.get_name());
    assert_eq!("refer", u.get_refer());
    let expected_err: Result<i32, String> = Err(String::from("err"));
    assert_eq!(expected_err, u.get_err().clone());
}
