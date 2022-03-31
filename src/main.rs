// Copyright (c) 2022  Teddy Wing
//
// This file is part of Yaqlite.
//
// Yaqlite is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Yaqlite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Yaqlite. If not, see <https://www.gnu.org/licenses/>.

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

    Update {
        #[clap(long)]
        database: String,

        #[clap(long, help = "Format: <column_name>=<primary-key>")]
        primary_key: String,

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
        exclude_column: Option<Vec<String>>,

        #[clap(long)]
        include_primary_key: bool,
    },
}


fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprint!("error");

            for cause in e.chain() {
                eprint!(": {}", cause);
            }

            eprintln!();

            std::process::exit(exitcode::SOFTWARE);
        }
    }
}

fn run() -> anyhow::Result<()> {
    use anyhow::Context;

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

            let mut dbconn = rusqlite::Connection::open(&database)
                .with_context(||
                    format!("can't connect to database '{}'", database)
                )?;

            let mut text_data;
            if input_file == "-" {
                use std::io::Read;

                text_data = String::new();
                std::io::stdin().read_to_string(&mut text_data)
                    .context("can't read from stdin")?;
            } else {
                text_data = std::fs::read_to_string(input_file)
                    .with_context(||
                        format!("can't read from file '{}'", input_file),
                    )?;
            }

            let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data)
                .context("can't parse YAML")?;

            yaqlite::insert(&mut dbconn, &table_name, &mut yaml_data)
                .context("failed to insert data")?;

            dbconn.close()
                .map_err(|e| {
                    let (_, err) = e;
                    err
                })
                .context("failed to close database")?;
        },

        Command::Update { .. } => {},

        Command::Select {
            database,
            table_name,
            primary_key,
            record_id,
            mut exclude_column,
            include_primary_key,
        } => {
            if exclude_column.is_none() && include_primary_key {
                exclude_column = Some(Vec::new());
            }

            let dbconn = rusqlite::Connection::open(&database)
                .with_context(||
                    format!("can't connect to database '{}'", database)
                )?;

            let yaml_data = match primary_key {
                Some(pk) => yaqlite::select_by_column(
                    &dbconn,
                    &table_name,
                    &pk,
                    &record_id,
                    exclude_column.as_deref(),
                ).with_context(||
                    format!("can't select record '{}'", record_id),
                )?,

                None => yaqlite::select(
                    &dbconn,
                    &table_name,
                    &record_id,
                    exclude_column.as_deref(),
                ).with_context(||
                    format!("can't select record '{}'", record_id),
                )?,
            };

            let stdout = std::io::stdout();
            let mut stdout_handle = stdout.lock();
            let mut buffer = yaqlite::yaml::IoAdapter::new(&mut stdout_handle);
            let mut emitter = yaml_rust::YamlEmitter::new(&mut buffer);
            emitter.multiline_strings(true);
            emitter.dump(&yaml_data)
                .context("can't serialize YAML")?;

            // YamlEmitter doesn't output a trailing newline.
            {
                use std::io::Write;
                writeln!(stdout_handle, "")
                    .context("failed to write to stdout")?;
            }

            dbconn.close()
                .map_err(|e| {
                    let (_, err) = e;
                    err
                })
                .context("failed to close database")?;
        },
    };

    Ok(())
}
