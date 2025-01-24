use actix_http::HttpMessage;
use actix_web::dev;
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use error_stack::report;

use crate::claims::UserClaims;
use crate::errors::AppError;
use crate::errors::AppResponseError;
use crate::token::Token;
use crate::AppState;


pub async fn bearer_validator<DB: 'static>(
    req: dev::ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<dev::ServiceRequest, (actix_web::error::Error, dev::ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((actix_web::error::ErrorBadRequest("No Bearer header"), req));
    };

    let Some(state) = req.app_data::<web::Data<AppState<DB>>>() else {
        let error: AppResponseError = report!(AppError)
            .attach_printable("No Application State")
            .into();
        return Err((error.into(), req));
    };

    let token = credentials.token();
    let user_claims: Result<UserClaims, _> = state.jwt.decode(&Token {
        token: token.to_string(),
    });

    match user_claims {
        Ok(user_claims) => {
            req.extensions_mut().insert(user_claims.id);
            Ok(req)
        }
        Err(error) => {
            let error: AppResponseError = error.into();
            Err((error.into(), req))
        }
    }
}
