use library::response::Response;
use poem::handler;
#[handler]
pub async fn login() -> Response<String> {
    Response::messsage(&String::from("123123"))
}
