use clap::Parser;
use rusqlite;
use yaml_rust::yaml;


#[derive(clap::Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Insert {
        #[clap(long)]
        database: String,

        table_name: String,

        input_file: Option<String>,
    },

    Select {
        #[clap(long)]
        database: String,

        table_name: String,

        #[clap(long)]
        primary_key: Option<String>,
        record_id: String,

        #[clap(long)]
        exclude_column: Vec<String>,
    },
}


fn main() {
    let args = Args::parse();

    match args.command {
        Command::Insert {
            database,
            table_name,
            input_file,
        } => {
            let input_file = match &input_file {
                Some(f) => f,
                None => "-",
            };

            let mut dbconn = rusqlite::Connection::open(database).unwrap();

            let mut text_data;
            if input_file == "-" {
                use std::io::Read;

                text_data = String::new();
                std::io::stdin().read_to_string(&mut text_data).unwrap();
            } else {
                text_data = std::fs::read_to_string(input_file).unwrap();
            }

            let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

            yaqlite::insert(&mut dbconn, &table_name, &mut yaml_data).unwrap();

            dbconn.close().unwrap();
        },

        Command::Select {
            database,
            table_name,
            primary_key,
            record_id,
            exclude_column,
        } => {
            let dbconn = rusqlite::Connection::open(database).unwrap();

            match primary_key {
                Some(pk) => yaqlite::select_by_column(
                    &dbconn,
                    &table_name,
                    &pk,
                    &record_id,
                ).unwrap(),

                None => yaqlite::select(
                    &dbconn,
                    &table_name,
                    &record_id,
                ).unwrap(),
            };

            dbconn.close().unwrap();
        },
    };
}
