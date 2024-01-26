use anyhow::{bail, Result};

use sqlite_starter_rust::database::Database;

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
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
