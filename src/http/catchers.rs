// use rocket::{http::Status, Catcher};
//
// pub fn catchers() -> Vec<Catcher> {
//     catchers![
//         not_found,
//         bad_request,
//         service_unavailable,
//         forbidden,
//         internal_server_error
//     ]
// }
//
// #[catch(400)]
// fn bad_request() -> &'static str {
//     Status::BadRequest.reason
// }
//
// #[catch(404)]
// fn not_found() -> &'static str {
//     Status::NotFound.reason
// }
//
// #[catch(403)]
// fn forbidden() -> &'static str {
//     Status::Forbidden.reason
// }
//
// #[catch(500)]
// fn internal_server_error() -> &'static str {
//     Status::InternalServerError.reason
// }
//
// #[catch(503)]
// fn service_unavailable() -> &'static str {
//     Status::ServiceUnavailable.reason
// }
