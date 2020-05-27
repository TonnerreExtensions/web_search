use crate::config::Config;
use crate::response::Response;
use crate::service::Service;
use crate::suggestion_proc;

pub fn query(request: &str, output: &str, identifier: &str, config: Config) {
    let (request, initial_service) = process(request, &config);
    if let Ok(initial_response) =
        Response::new(identifier, vec![initial_service]).serialize_to_json()
    {
        std::fs::write(output, initial_response).expect("OUTPUT closed");
    }
    let suggestions = suggestion_proc::process_suggestions(&config, request);
    if suggestions.is_empty() {
        return;
    }
    if let Ok(suggestion_response) = Response::new(identifier, suggestions).serialize_to_json() {
        std::fs::write(output, suggestion_response).expect("OUTPUT closed");
    }
}

fn process<'a>(request: &'a str, config: &Config) -> (&'a str, Service) {
    let split_index = match request.find(" ") {
        Some(index) => index,
        None => {
            return (
                "",
                build_service(env!("DEFAULT_URL"), env!("PROVIDER_NAME"), "..."),
            )
        }
    };
    let request = &request[split_index + 1..];
    (
        request,
        build_service(&config.search_url(request), request, request),
    )
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
            concat!("Search {} from ", env!("PROVIDER_NAME")),
            request.into()
        ),
    }
}
