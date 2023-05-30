use staticli::cmd_builder;

mod cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches: clap::ArgMatches = cmd_builder();

    // // 'list' command
    if let Some(_matches) = matches.subcommand_matches("list") {
        return cmd::list();
    }

    // // 'add' command
    if let Some(matches) = matches.subcommand_matches("add") {
        // "$ staticli add" was run
        let alias_opt = matches.get_one::<String>("alias");
        let endpoint_opt = matches.get_one::<String>("endpoint");

        // safely unwrap because these options are required
        let alias = alias_opt.unwrap();
        let endpoint = endpoint_opt.unwrap();
        println!(
            "Running command 'add' with alias: {}, and endpoint: {}",
            alias, endpoint
        );
        return cmd::add(&alias, &endpoint);
    }

    // // 'update' command
    if let Some(matches) = matches.subcommand_matches("update") {
        let alias_opt = matches.get_one::<String>("alias");
        let endpoint_opt = matches.get_one::<String>("endpoint");

        // safely unwrap because these options are required
        let alias = alias_opt.unwrap();
        let endpoint = endpoint_opt.unwrap();
        println!(
            "Running command 'update' with alias: {}, and endpoint: {}",
            alias, endpoint
        );
        return cmd::update(&alias, &endpoint);
    }

    // // 'delete' command
    if let Some(matches) = matches.subcommand_matches("delete") {
        let alias_opt = matches.get_one::<String>("alias");

        // safely unwrap because this option is required
        let alias = alias_opt.unwrap();
        return cmd::del(&alias);
    }

    // // 'check' command
    if let Some(matches) = matches.subcommand_matches("check") {
        let alias_opt = matches.get_one::<String>("alias");
        let endpoint_opt = matches.get_one::<String>("endpoint");

        if alias_opt.is_some() {
            return cmd::check_saved(alias_opt.unwrap());
        } else if endpoint_opt.is_some() {
            return cmd::check_unsaved(endpoint_opt.unwrap());
        } else {
            println!(
                "Missing option(s). Please either use 'alias' or 'endpoint' to run this command."
            );
        }
    }

    Ok(())
}
