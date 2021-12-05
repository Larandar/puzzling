use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use puzzling::prelude::*;
fn main() {
    // SECTION: BOOTSTRAP
    puzzling::logging::initialize_logging();
    // !SECTION

    let matches = App::new("advent")
        .about("Advent of Code utils cli")
        .author(crate_authors!())
        .version(crate_version!())
        .settings(&[
            AppSettings::ColorAuto,
            AppSettings::ColoredHelp,
            AppSettings::DeriveDisplayOrder,
            AppSettings::DisableHelpSubcommand,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommand(
            App::new("prepare")
                .about("Generate new bin using the templates/advent.rs file")
                .arg(
                    Arg::with_name("force")
                        .long("force")
                        .help("Overwrite existing file if present")
                        .takes_value(false)
                        .global(true),
                )
                .arg(
                    Arg::with_name("year")
                        .help("Year of Advent of Code")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("day")
                        .help("Day of Advent of Code")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            App::new("leaderboard")
                .about("Transform the leaderboard API to my liking")
                .arg(Arg::with_name("id")),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("prepare") => subcommand_prepare(matches),
        Some("leaderboard") => subcommand_leaderboard(matches),
        Some(_) => unreachable!("All subcommands MUST are described"),
        None => unreachable!("SubcommandRequiredElseHelp MUST prevent this branch"),
    };
}

fn subcommand_leaderboard(matches: ArgMatches) {
    let command = matches.subcommand_matches("leaderboard").unwrap();
    info!("{:?}", command)
}

fn subcommand_prepare(matches: ArgMatches) {
    let command = matches.subcommand_matches("prepare").unwrap();

    let year = command.value_of("year").unwrap();
    let day = command.value_of("day").unwrap();

    let cwd = std::env::current_dir().unwrap();

    let template_file = cwd.join("templates").join("advent").with_extension("rs");
    info!("Reading template file: {:?}", template_file);

    if !template_file.exists() {
        error!("Template file not found");
        return;
    }
    let contents = std::fs::read_to_string(template_file)
        .unwrap()
        .replace("YYYY", format!("{:04}", year).as_str())
        .replace("DD", format!("{:02}", day).as_str());

    let target_file = cwd
        .join("src")
        .join("bin")
        .join(format!("advent_{:04}_{:02}", year, day))
        .with_extension("rs");

    info!("Preparing file: {:?}", target_file);
    if target_file.exists() && !command.is_present("force") {
        error!("File already exists, to overwrite use --force flag");
        return;
    }

    std::fs::write(target_file, contents).unwrap()
}
