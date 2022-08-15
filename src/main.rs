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

    let server_version = match checks::server_version::check() {
        Ok(c) => c,
        Err(e) => panic!("❌ Failed to check kubectl server version: {}", e),
    };

    println!(
        "✅ kubectl version {}.{}",
        server_version.server_version.major, server_version.server_version.minor
    );

    let version_match:&str= if checks::version_match::check(cli_version.client_version, server_version.server_version) {"✅"} else {"❌"};

    println!("{} client and server versions match", version_match);
}
