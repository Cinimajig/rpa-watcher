#![allow(unused)]

use tiberius::{error::Error, Client, Config, FromSqlOwned, Query, Row};
use tokio::{io, net::TcpStream};
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type Database = Client<Compat<tokio::net::TcpStream>>;

pub static mut PRDB: Option<Database> = None;

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
    /// # Refactor
    /// This code needs some refactoring, but only after learning more about the `tiberius` crate.
    pub async fn query_instance(instance: &str) -> Result<Self, Error> {
        let mut sql_query = Query::new("SELECT instance_id, job_name, cause_text FROM job_instances_running WHERE instance_id = '@P1'");
        sql_query.bind(instance.to_string());

        let mut instance_id = String::new();
        let mut job_name = String::new();
        let mut cause_text = String::new();

        let row = query_row(sql_query).await?;
        for item in row
            .into_iter()
            .map(|s| String::from_sql_owned(s).unwrap_or_default())
            .enumerate()
        {
            match item {
                (0, Some(val)) => instance_id = val,
                (1, Some(val)) => job_name = val,
                (2, Some(val)) => cause_text = val,
                _ => return Err(Error::Io {
                    kind: io::ErrorKind::InvalidData,
                    message: "data not found or invalid".to_string(),
                }),
            }
        }

        Ok(Self {
            instance_id,
            job_name,
            cause_text,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.instance_id.is_empty()
    }
}

/// Tries to connect to a database, from a connection string.
pub async fn connect(connection_string: &str) -> Result<(), Error> {
    let mut config = Config::from_ado_string(connection_string)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    match Client::connect(config.clone(), tcp.compat_write()).await {
        Ok(client) => unsafe {
            PRDB = Some(client);
            Ok(())
        },
        // The server wants us to redirect to a different address.
        Err(Error::Routing { host, port }) => {
            config.host(&host);
            config.port(port);

            let tcp = TcpStream::connect(config.get_addr()).await?;
            tcp.set_nodelay(true)?;

            // we should not have more than one redirect, so we'll short-circuit here.
            unsafe {
                PRDB = Some(Client::connect(config, tcp.compat_write()).await?);
            }
            Ok(())
        }
        Err(e) => Err(e)?,
    }
}

pub async fn query_row<'a>(sql_query: Query<'a>) -> Result<Row, Error> {
    unsafe {
        match &mut PRDB {
            Some(client) => {
                let result = sql_query.query(client).await?;
                let row = result.into_row().await?;

                match row {
                    Some(val) => Ok(val),
                    None => Err(Error::Io {
                        kind: io::ErrorKind::NotFound,
                        message: "query returned nothing".to_string(),
                    }),
                }
            }
            None => Err(Error::Io {
                kind: io::ErrorKind::NotConnected,
                message: "not connected to database".to_string(),
            }),
        }
    }
}
