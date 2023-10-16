use crate::envoy::surveyor_envoy::SurveyorResponse;
use super::constants::{SCRIPT_DIR, CONFIG_DIR, BACKUP_CONFIG_DIR};
use std::process::Command;

#[derive(Debug, Clone)]
pub enum CommandStatus {
    Success,
    Failure,
    CouldNotExecute
}

#[derive(Debug, Clone)]
pub enum CommandName {
    MoveGrawFiles,
    BackupConfig,
    CheckRunExists
}

impl std::fmt::Display for CommandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveGrawFiles => write!(f, "MoveGrawFiles"),
            Self::BackupConfig => write!(f, "BackupConfig"),
            Self::CheckRunExists => write!(f, "CheckRunExists")
        }
    }
}

impl CommandName {
    pub fn get_function(&self) -> impl Fn(&[SurveyorResponse], &str, &i32) -> Result<CommandStatus, std::io::Error> {
        match self {
            Self::MoveGrawFiles => move_graw_files,
            Self::BackupConfig => backup_config,
            Self::CheckRunExists => check_run_exists
        }
    }
}

pub fn execute(command: CommandName, surveyor_data: &[SurveyorResponse], experiment: &str, run_number: &i32) -> CommandStatus {
    match command.get_function()(surveyor_data, experiment, run_number) {
        Ok(stat) => return stat,
        Err(e) => {
            tracing::error!("Could not execute command {}: {}", command, e);
            return CommandStatus::CouldNotExecute;
        }
    }
}

pub fn move_graw_files(surveyor_data: &[SurveyorResponse], experiment: &str, run_number: &i32) -> Result<CommandStatus, std::io::Error> {
    let sub_command = format!("{SCRIPT_DIR}move_graw.sh");
    let mut ret_stat = CommandStatus::Success;
    for data in surveyor_data {
        let output = Command::new("zsh")
                                .args([
                                    &sub_command, 
                                    &data.address, 
                                    &data.location, 
                                    experiment, 
                                    &(run_number.to_string())])
                                .output()?;
        if !output.status.success() {
            ret_stat = CommandStatus::Failure;
        }
    }
    Ok(ret_stat)
}

pub fn backup_config(_: &[SurveyorResponse], experiment: &str, run_number: &i32) -> Result<CommandStatus, std::io::Error> {
    let sub_command = format!("{SCRIPT_DIR}backup_configs.sh");
    let output = Command::new("zsh")
                                .args([
                                    &sub_command,
                                    CONFIG_DIR,
                                    BACKUP_CONFIG_DIR,
                                    experiment,
                                    &(run_number.to_string())
                                ])
                                .output()?;
    if output.status.success() {
        return Ok(CommandStatus::Success);
    } else {
        return Ok(CommandStatus::Failure);
    }
}

pub fn check_run_exists(surveyor_data: &[SurveyorResponse], experiment: &str, run_number: &i32) -> Result<CommandStatus, std::io::Error> {
    let sub_command = format!("{SCRIPT_DIR}test_graw.sh");
    let output = Command::new("zsh")
                                .args([
                                    &sub_command,
                                    &surveyor_data[0].address,
                                    &surveyor_data[0].location,
                                    experiment,
                                    &(run_number.to_string())
                                ])
                                .output()?;

    if output.status.success() {
        return Ok(CommandStatus::Success);
    } else {
        return Ok(CommandStatus::Failure);
    }
}