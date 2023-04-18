use rusqlite::{Connection, Result};

// Открыть базу, создать таблицу если ее нет
pub fn open(path: &String) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE if not exists banks (
            id INTEGER PRIMARY KEY,
            city text,
            name text,
            bik text,
            coor text
         )",
        (),
    )?;
    Ok(conn)
}

// Очистить таблицу
pub fn clear(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM banks", ())?;
    Ok(())
}
