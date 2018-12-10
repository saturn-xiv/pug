extern crate pug;

#[test]
fn it_migrations() {
    let cfg = pug::env::Config::default();
    let db = cfg.database().unwrap();
    let db = db.get().unwrap();
    pug::app::database::migrate::run(
        &db,
        &vec![
            pug::settings::migrations(),
            pug::i18n::db::migrations(),
            pug::nut::auth::migrations(),
            pug::nut::site::migrations(),
        ],
    )
    .unwrap();
}
