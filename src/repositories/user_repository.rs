use redb::{Database, DatabaseError, Error, ReadableDatabase, ReadableTable, TableDefinition};

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("user");


pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self{db}
    }

    pub fn insert_username(&self, username: &String, user_id: u64) -> Result<(), Error> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(username.as_str(), user_id)?;
        }

        write_txn.commit()?;
        Ok(())
    }

    pub fn get_user_id(&self, username: String) -> Result<Option<u64>, Error> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        match table.get(username.as_str())? {
            None => Ok(None),
            Some(x) => Ok(Some(x.value()))
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_insert_get(){
        let user_repository = UserRepository::new(Database::create("/tmp/test.redb").unwrap());

        user_repository.insert_username(&"teadove".to_string(), 42 ).unwrap();
        assert_eq!(Some(42), user_repository.get_user_id("teadove".to_string()).unwrap());
    }
}