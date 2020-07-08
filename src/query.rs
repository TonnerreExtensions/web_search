use crate::config::Config;
use crate::service::Service;
use crate::suggestion_proc;

pub fn query(request: &str, config: Config) {
    let initial_service = if request.is_empty() {
        build_service(config.main_url(), env!("PROVIDER_NAME"), "anything")
    } else {
        build_service(config.search_url(request), request, request)
    };
    if let Ok(initial_response) = initial_service.serialize_to_json() {
        println!("{}", initial_response);
    }
    if request.is_empty() {
        return;
    }
    suggestion_proc::process_suggestions(&config, request);
}

pub fn build_service<S: Into<String>, T: Into<String>, R: Into<String>>(
    id: S,
    title: T,
    request: R,
) -> Service {
    Service {
        id: id.into(),
        title: title.into(),
        subtitle: format!(
            concat!(r#"Search "{}" from "#, env!("PROVIDER_NAME")),
            request.into()
        ),
    }
}
