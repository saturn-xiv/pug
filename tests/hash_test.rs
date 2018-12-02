extern crate pug;

#[test]
fn it_migrations() {
    for it in vec![
        "MyEmailAddress@example.com ",
        " MyEmailAddress@example.com",
        " myemailaddress@example.com ",
        "myemailaddress@example.com",
    ] {
        assert_eq!(
            pug::nut::auth::user::gravatar_hash(&it),
            "0bc83cb571cd1c50ba6f3e8a78ef1346".to_string()
        );
    }
}
