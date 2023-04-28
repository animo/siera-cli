use serde::Deserialize;
use siera_agent::modules::connection::Connection;

#[macro_use]
mod macros;

mod utils;

#[derive(Deserialize)]
struct ConnectionListWrapper {
    connections: Vec<Connection>,
}

#[derive(Deserialize)]
struct BasicMessageWrapper {
    message: String,
}

#[derive(Deserialize)]
struct InvitationWrapper {
    invitation_url: String,
}

#[derive(Deserialize)]
struct ConnectionIdWrapper {
    connection_id: String,
}

#[cfg(test)]
mod e2e_tests {
    use crate::{
        BasicMessageWrapper, ConnectionIdWrapper, ConnectionListWrapper, InvitationWrapper,
    };

    use super::utils::helpers::{run_test, REGEX_UUID};
    use regex::Regex;
    use speculoos::prelude::*;

    test! { smoke |cli| {
        let re = Regex::new(r"^siera \d\.\d.\d").unwrap();
        let ret = siera!(cli, "--version");
        assert_that(&ret).matches(|v| re.is_match(v));
    }}

    test! { create_a_connection_and_list_connections |cli| {
        let connections = siera!(cli, "connection list");
        let connections: ConnectionListWrapper = serde_json::from_str(&connections).unwrap();
        assert_that(&connections.connections).has_length(0);
        siera!(cli, "connection invite");
        let connections = siera!(cli, "connection list");
        let connections: ConnectionListWrapper = serde_json::from_str(&connections).unwrap();
        assert_that(&connections.connections).has_length(1);
    }}

    test! { create_connection_and_send_a_message |cli| {
        siera!(cli, "connection invite");
        let connections_str = siera!(cli, "connection list");
        let connection: ConnectionListWrapper = serde_json::from_str(&connections_str).unwrap();
        let result = siera!(cli, "message --connection-id={} --message={}", connection.connections[0].id, "bar");
        let result:BasicMessageWrapper = serde_json::from_str(&result).unwrap();
        assert_that(&result.message).is_equal_to(String::from("Successfully sent message"));
    }}

    test! { create_invitation_and_receive_invitation |cli| {
         let invitation_str = siera!(cli, "connection invite");
         let invitation_str: Vec<&str> = invitation_str.split_inclusive("}").collect();
         let invitation_str = &invitation_str[3];
         let invitation: InvitationWrapper = serde_json::from_str(invitation_str).unwrap();
         let result = siera!(cli, "connection receive --url={}", invitation.invitation_url);
         let result: Vec<&str> = result.split_inclusive("}").collect();
         let result: ConnectionIdWrapper = serde_json::from_str(&result[1].replace("\n", "")).unwrap();
         let connection_id = result
             .connection_id;
         let re = Regex::new(REGEX_UUID).unwrap();
         assert_that(&connection_id).matches(|c| re.is_match(c) );
    }}
}
