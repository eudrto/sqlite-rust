use std::process::exit;

use anyhow::{bail, Result};

use sqlite_starter_rust::{database::Database, sql::SelectStmt};

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
            let tables = db.load_sqlite_schema_table().dot_tables();
            println!("{tables}");
        }
        _ => {
            let stmt = SelectStmt::parse(command);

            let table = match db.load_table(&stmt.from) {
                Ok(table) => table,
                Err(msg) => {
                    println!("{}", msg);
                    exit(1);
                }
            };

            if stmt.select.to_lowercase() == "count(*)" {
                println!("{}", table.size())
            } else {
                println!("{}", table.select(&[stmt.select]));
            }
        }
    }

    Ok(())
}
