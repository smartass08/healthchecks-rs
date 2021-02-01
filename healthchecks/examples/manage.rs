use healthchecks::errors::HealthchecksApiError;
use healthchecks::manage::get_client;
use healthchecks::model::NewCheck;
use std::result::Result;

fn main() -> Result<(), HealthchecksApiError> {
    let api_key = std::env::args()
        .nth(1)
        .expect("Providing an API key as the first parameter is required");
    let config = get_client(api_key, None).unwrap();
    for check in config.get_checks()? {
        println!("{:?}", check);
    }
    let new_check: NewCheck = Default::default();
    println!("{:?}", config.create_check(new_check)?);
    Ok(())
}
