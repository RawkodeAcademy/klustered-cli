mod checks;

fn main() {
    let cli_version = match checks::cli_version::check() {
        Ok(c) => c,
        Err(e) => panic!("❌ Failed to check kubectl client version: {}", e),
    };

    println!(
        "✅ kubectl version {}.{}",
        cli_version.client_version.major, cli_version.client_version.minor
    );
}
