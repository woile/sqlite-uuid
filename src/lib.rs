use sqlite_loadable::prelude::*;

use sqlite_loadable::{api, define_scalar_function, Result};

/// Create a TEXT UUIDv4
///
/// # Example
///
/// ```sql
/// SELECT uuid(); -- v4
/// ```
pub fn uuid(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let val = uuid::Uuid::new_v4().to_string();
    api::result_text(context, val)?;
    Ok(())
}

/// Create a TEXT UUIDv7
///
/// # Example
///
/// ```sql
/// SELECT uuid_v7(); -- v7
/// ```
pub fn uuid_v7(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let val = uuid::Uuid::now_v7().to_string();
    api::result_text(context, val)?;
    Ok(())
}

/// Create a blob UUIDv4
///
/// ## Example
///
/// ```sql
/// SELECT uuid_blob(); -- 4
/// ```
pub fn uuid_blob(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let val = uuid::Uuid::new_v4();
            api::result_blob(context, val.as_bytes());
    Ok(())
}

/// Create a blob UUIDv7
///
/// ## Example
///
/// ```sql
/// SELECT uuid_v7_blob(); -- v7
/// ```
pub fn uuid_v7_blob(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let val = uuid::Uuid::now_v7();
    api::result_blob(context, val.as_bytes());
    Ok(())
}

/// Convert a blob to a string representation of a UUID
///
/// It doesn't matter if it's a v4 or v7 UUID, it will be converted to a string
///
/// ## Example
///
/// ```sql
/// SELECT uuid_from_blob(user_id) from users;
/// ```
pub fn uuid_from_blob(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let v = api::value_blob(values.get(0).expect("Failed to get blob value"));
    let uuid = uuid::Uuid::from_slice(v.as_ref()).expect("Invalid uuid blob");
    api::result_text(context, uuid.to_string())?;
    Ok(())
}

/// Convert a string representation of a UUID to blob
///
/// It doesn't matter if it's a v4 or v7 UUID, it will be converted to a string
///
/// ## Example
///
/// ```sql
/// SELECT uuid_as_blob(user_id) from users;
///
/// --- or for inserting
/// INSERT INTO events(event_id) VALUES (uuid_as_blob('018d9887-42cd-7115-b1ca-18227ac211b4'));
/// ```
pub fn uuid_as_blob(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let v = api::value_text(values.get(0).expect("Failed to get text value"))?;
    let uuid = uuid::Uuid::parse_str(v).expect("Invalid uuid string");
    api::result_blob(context, uuid.as_bytes());
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_sqliteuuid_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "uuid", 0, uuid, FunctionFlags::UTF8)?;
    define_scalar_function(db, "uuid_v7", 0, uuid_v7, FunctionFlags::UTF8)?;
    define_scalar_function(db, "uuid_blob", 0, uuid_blob, FunctionFlags::UTF8)?;
    define_scalar_function(db, "uuid_v7_blob", 0, uuid_v7_blob, FunctionFlags::UTF8)?;
    define_scalar_function(
        db,
        "uuid_from_blob",
        1,
        uuid_from_blob,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;
    define_scalar_function(
        db,
        "uuid_as_blob",
        1,
        uuid_as_blob,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {}
