#![allow(unused)]

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tiberius::{error::Error, Client, Config, FromSqlOwned, Query, Row, SqlBrowser, Uuid};
use tokio::{io, net::TcpStream, sync::{RwLock, RwLockWriteGuard}};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type Database = Client<Compat<tokio::net::TcpStream>>;

/// An instance of a running job in the `job_instances_running` table.
#[derive(Debug)]
pub struct ProcessRobotJob {
    pub instance_id: String,
    pub job_name: String,
    pub cause_text: String,
}

impl ProcessRobotJob {
    /// Queries a running instance of ProcessRobot in the database.
    ///
    /// The data might be empty, just in case it returned noting, but shouldn't be possible.
    pub async fn query_instance(db: &mut crate::db::Database, instance: &str) -> Result<Self, Error> {
        let mut sql_query = Query::new("SELECT instance_id, job_name, cause_text FROM job_instances_running WHERE instance_id = @P1");
        sql_query.bind(instance);

        let mut client = db;
        let stream = sql_query.query(&mut client).await?;
        let row = stream.into_row().await?;

        match row {
            Some(row) => Ok(Self {
                instance_id: row
                    .get::<'_, Uuid, _>("instance_id")
                    .map(String::from)
                    .unwrap_or(instance.to_string()),
                job_name: row
                    .get::<'_, &str, _>("job_name")
                    .map(String::from)
                    .unwrap_or_default(),
                cause_text: row
                    .get::<'_, &str, _>("cause_text")
                    .map(String::from)
                    .unwrap_or_default(),
            }),
            None => Err(Error::Io {
                kind: io::ErrorKind::NotFound,
                message: "no data was returned".to_string(),
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.instance_id.is_empty()
    }
}

// impl DBWrapper {
    /// Tries to connect to a database, from a connection string.
    pub async fn create(connection_string: &str) -> Result<Arc<RwLock<Database>>, Error> {
        let mut config = Config::from_ado_string(connection_string)?;

        let tcp = TcpStream::connect_named(&config).await?;
        tcp.set_nodelay(true)?;

        match Client::connect(config.clone(), tcp.compat_write()).await {
            Ok(client) => Ok((Arc::new(RwLock::new(client)))),
            // The server wants us to redirect to a different address.
            Err(Error::Routing { host, port }) => {
                config.host(&host);
                config.port(port);

                let tcp = TcpStream::connect(config.get_addr()).await?;
                tcp.set_nodelay(true)?;

                // we should not have more than one redirect, so we'll short-circuit here.
                Ok((Arc::new(RwLock::new(Client::connect(config, tcp.compat_write()).await?))))
            }
            Err(e) => Err(e),
        }
    }
// }
