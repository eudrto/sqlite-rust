use anyhow::{bail, Result};

use sqlite_starter_rust::engine::Database;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    let file_path = &args[1];
    let cmd = &args[2];

    let mut db = Database::new(&file_path);
    db.exec(cmd);

    Ok(())
}
