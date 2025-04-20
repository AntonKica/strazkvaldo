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

pub async fn check_role_middleware(
    req: ServiceRequest,
    next: Next<impl actix_web::body::MessageBody + 'static>,
    required_role: String,
) -> Result<ServiceResponse<EitherBody<impl actix_web::body::MessageBody>>, Error> {
    let session = req.get_session();

    // Then check role
    if let Some(role) = session.get::<String>("role")? {
        if has_required_role(&role, &required_role) {
            return next
                .call(req)
                .await
                .map(ServiceResponse::map_into_left_body);
        }
    }

    // If role check fails
    let (request, _pl) = req.into_parts();
    Ok(ServiceResponse::new(
        request,
        HttpResponse::Forbidden()
            .body("Insufficient permissions")
            .map_into_right_body(),
    ))
}

fn has_required_role(user_role: &String, required_role: &String) -> bool {
    match (user_role.as_str(), required_role.as_str()) {
        ("admin", _) => true, // Admins can access anything
        ("user", "user") => true,
        _ => false,
    }
}
