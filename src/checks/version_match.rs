use super::{cli_version::ClientVersion, server_version::ServerVersion};

pub fn check(client: ClientVersion, server: ServerVersion) -> bool{
  client.major == server.major && client.minor == client.minor  
}