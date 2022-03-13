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
    println!("Hello, world!");

    let args = Args::parse();

    // Get column names from SQLite

    match args.command {
        a @ Command::Insert { .. } => {
            dbg!(a.database);
        }

        a @ Command::Select { .. } => {}
    };

    let mut dbconn = rusqlite::Connection::open("./test.db").unwrap();

    let text_data = std::fs::read_to_string("test2.yml").unwrap();

    let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

    yaqlite::insert(&mut dbconn, "people", &mut yaml_data).unwrap();

    dbg!(yaml_data);

    dbconn.close().unwrap();
}
