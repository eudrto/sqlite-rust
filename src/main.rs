use anyhow::{bail, Result};
use std::fs::File;

use sqlite_starter_rust::{db_header::DBHeader, page_header::PageHeader};

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;

            let db_header = DBHeader::new(&mut file);
            let page_header = PageHeader::new(&mut file);

            println!("database page size: {}", db_header.page_size);
            println!("number of tables: {}", page_header.cell_cnt);
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
