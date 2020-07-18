use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
	id: i32,
	name: String,
	data: Option<Vec<u8>>,
}

fn create_tables(conn: &Connection) {
	let sql = "CREATE TABLE person (
			id      INTEGER PRIMARY KEY,
			name    TEXT NOT NULL,
			data    BLOB
	)";
	match conn.execute(sql, params![]) {
		Ok(_) => (),
		Err(_) => return,
	}
}

fn main() -> Result<()> {
	// let conn = Connection::open_in_memory()?;
	let path = "./my_db.sqlite";
	let conn = Connection::open(&path)?;

	create_tables(&conn);


	let me = Person {
		id: 0,
		name: "Steven".to_string(),
		data: None,
	};
	
	conn.execute(
		"INSERT INTO person (name, data) VALUES (?1, ?2)",
		params![me.name, me.data],
	)?;

	conn.execute(
		"INSERT INTO person (name, data) VALUES (?1, ?2)",
		params!["SQLite is nice".to_string(), Some(vec![20u8, 33u8])],
	)?;

	let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
	let person_iter = stmt.query_map(params![], |row| {
		Ok(Person {
			id: row.get(0)?,
			name: row.get(1)?,
			data: row.get(2)?,
		})
	})?;

	for person in person_iter {
		println!("Found person {:?}", person.unwrap());
	}

	Ok(())
}
