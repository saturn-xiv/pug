extern crate pug;

#[test]
fn it_system() {
    println!("hostname: {}", pug::sys::hostname().unwrap());
    println!("pid: {}", pug::sys::pid());

    let uts_name = pug::sys::uts_name();
    println!(
        "uts name: {} {} {} {} {}",
        uts_name.machine(),
        uts_name.nodename(),
        uts_name.release(),
        uts_name.sysname(),
        uts_name.version()
    );

    let sys_info = pug::sys::sys_info().unwrap();
    println!(
        "sys uptime: {:?} cpu number {}",
        sys_info.uptime(),
        sys_info.process_count()
    );
}

#[test]
fn it_network() {
    for it in pug::sys::network::interfaces().unwrap() {
        println!(
            "interface {}:\t{:?}\t{:?}\t{}",
            it,
            pug::sys::network::ip4(&it).unwrap(),
            pug::sys::network::ip6(&it).unwrap(),
            pug::sys::network::mac(&it)
                .unwrap()
                .unwrap()
                .to_hex_string()
        );
    }
}
