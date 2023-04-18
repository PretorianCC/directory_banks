use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct RecordBank {
    pub city: String,
    pub name: String,
    pub bik: String,
    pub coor: String,
}

impl RecordBank {
    pub fn add(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO banks (city, name, bik, coor) VALUES (?1, ?2, ?3, ?4)",
            (&self.city, &self.name, &self.bik, &self.coor),
        )?;
        Ok(())
    }
}
