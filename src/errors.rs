error_chain!{

    foreign_links {
        JsonWebToken(jsonwebtoken::errors::Error);
        SerdeJson(serde_json::Error);
        RedisError(rocket_contrib::databases::redis::RedisError);
    }

}
