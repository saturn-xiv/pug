error_chain! {

    foreign_links {
        StdIo(std::io::Error);
        StdSystemTime(std::time::SystemTimeError);
        StdStrUtf8(std::str::Utf8Error);
        StdNumParseInt(std::num::ParseIntError);
        StdStringFromUtf8(std::string::FromUtf8Error);

        Log4rs(log4rs::Error);
        JsonWebToken(jsonwebtoken::errors::Error);
        SerdeJson(serde_json::Error);
        Base64Decode(base64::DecodeError);
        TomlDe(toml::de::Error);
        TomlSer(toml::ser::Error);
        Hyper(hyper::Error);
        HyperHeaderToStr(hyper::header::ToStrError);
        LanguageTags(language_tags::Error);
        ChronoParse(chrono::ParseError);
        XmlReader(xml::reader::Error);
        SerdeXmlRs(serde_xml_rs::Error);
        Diesel(diesel::result::Error);
        Mustache(mustache::Error);
        R2d2(r2d2::Error);
        UrlParse(url::ParseError);
        CookieParse(cookie::ParseError);
        RocketConfig(rocket::config::ConfigError);

        ZeroMq(zmq::Error);
        Redis(r2d2_redis::redis::RedisError);
    }

}
