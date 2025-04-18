use actix_session::{Session, SessionExt};
use actix_web::body::EitherBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{Error, HttpResponse};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody + 'static>,
) -> Result<ServiceResponse<EitherBody<impl actix_web::body::MessageBody>>, Error> {
    // this one is public for sure
    if req.path().eq("/svc/auth/login") {
        return next
            .call(req)
            .await
            .map(ServiceResponse::map_into_left_body);
    }

    match req.cookie("id") {
        Some(cookie) => {
            println!("--------- The cookie is {:?}", cookie.value().to_string());
        }
        None => {
            println!("--------- The cookie is empty");
        }
    };

    let session = req.get_session();

    if !is_authenticated(&session) {
        let (request, _pl) = req.into_parts();

        // Create a response with 401 status and Location header
        let response = HttpResponse::Unauthorized()
            .append_header(("Location", "/ui/login"))
            .finish()
            .map_into_right_body();

        return Ok(ServiceResponse::new(request, response));
    }

    next.call(req)
        .await
        .map(ServiceResponse::map_into_left_body)
}
pub fn is_authenticated(session: &Session) -> bool {
    match session.get::<String>("code") {
        Ok(value) => value.is_some(),
        Err(_) => false,
    }
}
