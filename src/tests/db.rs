use crate::db::db_struct::{DBValue, DB};

#[test]
pub fn db() {
	let mut db = DB::new("RUST_TESTS");
	db.clear();
	assert_eq!(db.get("x"), None);
	db.set("x", DBValue::Num(1.0));
	assert_eq!(db.get("x"), Some(&DBValue::Num(1.0)));

	let db2 = DB::new("RUST_TESTS");
	assert_eq!(db2.get("x"), db.get("x"));
	assert_eq!(db.data.len(), 1);
	assert_eq!(db2.data.len(), 1);
	assert_eq!(db.get("x"), db2.data.get("x"));
}
