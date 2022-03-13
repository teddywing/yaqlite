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

        record_id: String,
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
            let mut dbconn = rusqlite::Connection::open(database).unwrap();

            let text_data = std::fs::read_to_string(input_file.unwrap()).unwrap();

            let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

            yaqlite::insert(&mut dbconn, &table_name, &mut yaml_data).unwrap();

            dbg!(yaml_data);

            dbconn.close().unwrap();
        },

        Command::Select {
            database,
            table_name,
            record_id,
        } => {},
    };
}
