//! # sg-tools
//!
//! `sg-tools` is a simple CLI tool to perform routine checks on AWS Security Groups,
//! especially to find and document common misconfigurations.
//!
//! ## Usage
//!
//! ```shell
//! sg-tools -o list
//! ```


use clap::{Arg, command, Command};
use aws_sdk_ec2::Client as Ec2Client;
use clap::builder::PossibleValue;

mod list_sg;
mod check_cidrs;

/// Create the main command for the CLI
pub fn sg_tools_command() -> Command {
    command!("sg-tools")
        .version("0.1.0")
        .about("Tools for managing the AWS Security Groups")
        .author("Antonio Masotti <toniomasotti@gmail.com>")
        .arg(
            Arg::new("operation")
                .short('o')
                .long("operation")
                .help("Operation to perform")
                .required(true)
                .value_parser([
                    PossibleValue::new("list"),
                    PossibleValue::new("describe"),
                    PossibleValue::new("find-default-ingress"),
                    PossibleValue::new("find-default-egress"),
                    PossibleValue::new("find-all-ports"),
                ])
        )
        .arg(
            Arg::new("profile")
                .short('p')
                .long("profile")
                .help("AWS Profile to use")
                .default_missing_value("default")
                .default_value("default")
        )
        .arg(
            Arg::new("region")
                .short('r')
                .long("region")
                .help("AWS Region to use")
                .default_value("eu-central-1")
                .default_missing_value("eu-central-1")
        )
}

/// Process the command passed as argument and execute the corresponding function
///
/// # Arguments
///
/// * `args` - The arguments passed to the CLI
/// * `client` - The EC2 client to use
///
/// # Panics
///
/// If the operation is not valid
///
/// # Available operations
///
/// * `list` - List all the security groups
/// * `describe` - Describe all the security groups
/// * `find-default-ingress` - Find security groups with default ingress rule (0.0.0.0/0)
/// * `find-default-egress` - Find security groups with default egress rule (0.0.0.0/0)
/// * `find-all-ports` - Find security groups with all ports open
///
pub async fn process_command(args: &clap::ArgMatches, client: Ec2Client) {

    let operation = args.get_one::<String>("operation")
        .expect("Operation is required");

    match operation.as_str() {
        "list" => list_sg::list_security_groups(&client).await,
        "describe" => check_cidrs::describe_all_sg(client).await,
        "find-default-ingress" => check_cidrs::find_sg_default_ingress(client).await,
        "find-default-egress" => check_cidrs::find_sg_default_egress(client).await,
        "find-all-ports" => check_cidrs::find_sg_all_ports(client).await,
        _ => panic!("Invalid operation")
    }
}
