mod client;

fn main() -> Result<(), druid::PlatformError> {
    client::run()
}
