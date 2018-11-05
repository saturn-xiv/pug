error_chain!{

    foreign_links {
        StdIo(std::io::Error);

        JsonWebToken(jsonwebtoken::errors::Error);
        SerdeJson(serde_json::Error);

        RedisError(rocket_contrib::databases::redis::RedisError) #[cfg(feature = "redis")];
    }

}
