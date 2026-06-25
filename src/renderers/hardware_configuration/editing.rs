use ratatui::widgets::ListState;

use crate::ftc_proto::command_packet::RobotConfigurationFile;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditingData {
    pub config: RobotConfigurationFile,
    pub list_state: ListState,
    pub config_data: Option<crate::ftc_proto::hardware::robot::Robot>,
}
