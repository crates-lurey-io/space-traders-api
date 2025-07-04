/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.3.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SystemWaypoint : Waypoint details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemWaypoint {
    /// The symbol of the waypoint.
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "type")]
    pub r#type: models::WaypointType,
    /// Relative position of the waypoint on the system's x axis. This is not an absolute position in the universe.
    #[serde(rename = "x")]
    pub x: i32,
    /// Relative position of the waypoint on the system's y axis. This is not an absolute position in the universe.
    #[serde(rename = "y")]
    pub y: i32,
    /// Waypoints that orbit this waypoint.
    #[serde(rename = "orbitals")]
    pub orbitals: Vec<models::WaypointOrbital>,
    /// The symbol of the parent waypoint, if this waypoint is in orbit around another waypoint. Otherwise this value is undefined.
    #[serde(rename = "orbits", skip_serializing_if = "Option::is_none")]
    pub orbits: Option<String>,
}

impl SystemWaypoint {
    /// Waypoint details.
    pub fn new(symbol: String, r#type: models::WaypointType, x: i32, y: i32, orbitals: Vec<models::WaypointOrbital>) -> SystemWaypoint {
        SystemWaypoint {
            symbol,
            r#type,
            x,
            y,
            orbitals,
            orbits: None,
        }
    }
}

