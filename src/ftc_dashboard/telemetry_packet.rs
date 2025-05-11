use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/telemetry/TelemetryPacket.java#L14>
pub struct TelemetryPacket {
    pub timestamp: i64,
    pub data: HashMap<String, String>,
    pub log: Vec<String>,
    // TODO: add canvas
}
