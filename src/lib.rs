use clap::{arg, command, value_parser, ArgAction, Command};

pub fn cmd_builder() -> clap::ArgMatches {
    command!()
        .subcommand(Command::new("list").about("List stored aliases and the endpoints associated"))
        .subcommand(
            Command::new("add")
                .about("Add new alias")
                .arg(
                    arg!(-a --alias <ALIAS> "Name of alias")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-e --endpoint <ENDPOINT> "Endpoint of alias")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update existing alias")
                .arg(
                    arg!(-a --alias <ALIAS> "Name of alias")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-e --endpoint <ENDPOINT> "Endpoint of alias")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("delete").about("Delete existing alias").arg(
                arg!(-a --alias <ALIAS> "Name of alias")
                    .action(ArgAction::Set)
                    .required(true)
                    .value_parser(value_parser!(String)),
            ),
        )
        .subcommand(
            Command::new("check")
                .about("Run a status check against a given alias or endpoint")
                .arg(
                    arg!(-a --alias <ALIAS> "Name of alias")
                        .action(ArgAction::Set)
                        .required(false)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(-e --endpoint <ENDPOINT> "Endpoint")
                        .action(ArgAction::Set)
                        .required(false)
                        .value_parser(value_parser!(String)),
                ),
        )
        // .subcommand(
        //     Command::new("tail")
        //         .about("Run a set number of checks against a given alias on an interval")
        //         .arg(arg!(-a --alias "Name of alias").action(ArgAction::SetTrue))
        //         .arg(arg!(-e --endpoint "Endpoint").action(ArgAction::SetTrue))
        //         .arg(arg!(-i --interval "Interval in seconds").action(ArgAction::SetTrue))
        //         .arg(arg!(-c --count "Number of checks to run").action(ArgAction::SetTrue))
        //         .arg(arg!(-f --file "File to output results to").action(ArgAction::SetTrue)),
        // )
        .get_matches()
}
