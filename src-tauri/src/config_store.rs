use directories::UserDirs;
use microkv::MicroKV;
use rand::{self, RngCore};
use rusqlite::{Connection, Result, Statement};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct ConfigStoreState(pub Mutex<ConfigStore>);
pub struct ConfigStore {
  _sql_conn: Connection,
  _db: MicroKV,
}

#[derive(Debug)]
struct KeyStore {
  key: Vec<u8>,
}

impl ConfigStore {
  pub fn init() -> ConfigStore {
    let config_dir = get_config_dir();
    fs::create_dir_all(config_dir.clone()).unwrap();
    let connection = Connection::open(config_dir.clone().join("key.db")).unwrap();

    let key: [u8; 32] =
      <[u8; 32]>::try_from(init_sqlite_store(&connection).unwrap().as_slice()).unwrap();

    let db_path = config_dir.clone();

    let db = match db_path.try_exists() {
      Ok(_) => MicroKV::open_with_base_path("email-print-server-db", db_path)
        .unwrap()
        .with_pwd_hash(key)
        .set_auto_commit(true),
      Err(_) => MicroKV::new_with_base_path("email-print-server-db", db_path)
        .with_pwd_hash(key)
        .set_auto_commit(true),
    };

    ConfigStore {
      _sql_conn: connection,
      _db: db,
    }
  }

  pub fn get_value<T>(&self, name: &str) -> Option<T>
  where
    T: DeserializeOwned + 'static,
  {
    self._db.get::<T>(name).unwrap()
  }

  pub fn store_value<T: Serialize>(&self, name: &str, value: T) {
    self._db.put(name, &value).unwrap();
  }
}

fn init_sqlite_store(connection: &Connection) -> Result<Vec<u8>, ()> {
  let statement = connection.prepare("SELECT key FROM meta WHERE id='key'");

  let res = match statement {
    Ok(stmt) => get_key(stmt),
    Err(_) => setup_sqlite(connection).unwrap(),
  };

  Ok(res.key)
}

fn get_key(mut statement: Statement) -> KeyStore {
  let mut key_iter = statement
    .query_map([], |row| {
      Ok(KeyStore {
        key: row.get("key")?,
      })
    })
    .unwrap();

  key_iter.next().unwrap().unwrap()
}

fn setup_sqlite(connection: &Connection) -> Result<KeyStore, rusqlite::Error> {
  connection.execute(
    "CREATE TABLE meta (
    id STRING PRIMARY KEY,
    key BLOB
  )",
    (),
  )?;

  let mut data = [0u8; 32];
  rand::thread_rng().fill_bytes(&mut data);

  connection.execute("INSERT INTO meta (id, key) VALUES (?1, ?2)", ("key", &data))?;

  Ok(KeyStore {
    key: Vec::from(data),
  })
}

fn get_config_dir() -> PathBuf {
  let user_dirs = UserDirs::new();
  return user_dirs.unwrap().home_dir().join(".email-print-server");
}
