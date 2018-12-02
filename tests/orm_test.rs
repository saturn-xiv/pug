extern crate pug;

#[test]
fn it_migrations() {
    let cfg = pug::env::Config::default();
    let db = cfg.database().unwrap();
    let db = db.get().unwrap();
    pug::app::database::migrate::run(
        &db,
        &vec![
            pug::settings::migration(),
            pug::i18n::locales::migration(),
            pug::nut::auth::migration(),
            pug::nut::site::migration(),
        ],
    )
    .unwrap();
}
