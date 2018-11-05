error_chain!{

    foreign_links {
        JsonWebToken(jsonwebtoken::errors::Error);
    }

}
