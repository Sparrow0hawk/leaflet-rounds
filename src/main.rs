use env_logger::Env;
use leaflet_rounds::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let settings = leaflet_rounds::configuration::get_configuration().expect("Unable to parse configuration file");

    let address = format!("{}:{}", settings.host, settings.port);
    println!("App listening on http://{}", &address);

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
