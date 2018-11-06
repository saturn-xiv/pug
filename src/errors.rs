error_chain!{

    foreign_links {
        StdIo(std::io::Error);
        StdSystemTime(std::time::SystemTimeError);
        StdStrUtf8(std::str::Utf8Error);
        StdNumParseInt(std::num::ParseIntError);
        StdStringFromUtf8(std::string::FromUtf8Error);

        JsonWebToken(jsonwebtoken::errors::Error);
        SerdeJson(serde_json::Error);
        Base64Decode(base64::DecodeError);
        TomlDe(toml::de::Error);
        TomlSer(toml::ser::Error);
        RocketConfig(rocket::config::ConfigError);
        Hyper(hyper::Error);
        RocketTera(rocket_contrib::templates::tera::Error);
        ChronoParse(chrono::ParseError);
        XmlReader(xml::reader::Error);
        SerdeXmlRs(serde_xml_rs::Error);

        RedisError(rocket_contrib::databases::redis::RedisError) #[cfg(feature = "redis")];
    }

}
