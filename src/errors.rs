error_chain!{

    foreign_links {
        StdIo(std::io::Error);

        JsonWebToken(jsonwebtoken::errors::Error);
        SerdeJson(serde_json::Error);
        Base64Decode(base64::DecodeError);
        TomlDe(toml::de::Error);
        TomlSer(toml::ser::Error);
        RocketConfig(rocket::config::ConfigError);
        RocketTera(rocket_contrib::templates::tera::Error);

        RedisError(rocket_contrib::databases::redis::RedisError) #[cfg(feature = "redis")];
    }

}
