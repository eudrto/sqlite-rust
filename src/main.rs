use std::process::exit;

use anyhow::{bail, Result};

use sqlite_starter_rust::{database::Database, sql::SelectStmt, sqlite_schema::SQLiteObjectType};

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let mut db = Database::new(&args[1]);

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            println!("database page size: {}", db.header.page_size);
            println!("number of tables: {}", db.read_page(1).header.cell_cnt);
        }
        ".tables" => {
            let tables = db
                .load_sqlite_schema_table()
                .sqlite_objects
                .into_iter()
                .filter(|sqlite_object| {
                    if let SQLiteObjectType::Table = sqlite_object.object_type {
                        true
                    } else {
                        false
                    }
                })
                .map(|sqlite_object| sqlite_object.name)
                .reduce(|acc, e| acc + " " + &e)
                .unwrap();

            println!("{tables}");
        }
        _ => {
            let stmt = SelectStmt::parse(command);

            let root_page = match db.load_root_page(&stmt.from) {
                Ok(root_page) => root_page,
                Err(msg) => {
                    println!("{}", msg);
                    exit(1);
                }
            };

            println!("{}", root_page.header.cell_cnt)
        }
    }

    Ok(())
}
