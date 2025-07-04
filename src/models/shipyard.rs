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

/// Shipyard : Shipyard details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Shipyard {
    /// The symbol of the shipyard. The symbol is the same as the waypoint where the shipyard is located.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The list of ship types available for purchase at this shipyard.
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<models::ShipyardShipTypesInner>,
    /// The list of recent transactions at this shipyard.
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<models::ShipyardTransaction>>,
    /// The ships that are currently available for purchase at the shipyard.
    #[serde(rename = "ships", skip_serializing_if = "Option::is_none")]
    pub ships: Option<Vec<models::ShipyardShip>>,
    /// The fee to modify a ship at this shipyard. This includes installing or removing modules and mounts on a ship. In the case of mounts, the fee is a flat rate per mount. In the case of modules, the fee is per slot the module occupies.
    #[serde(rename = "modificationsFee")]
    pub modifications_fee: i32,
}

impl Shipyard {
    /// Shipyard details.
    pub fn new(symbol: String, ship_types: Vec<models::ShipyardShipTypesInner>, modifications_fee: i32) -> Shipyard {
        Shipyard {
            symbol,
            ship_types,
            transactions: None,
            ships: None,
            modifications_fee,
        }
    }
}

