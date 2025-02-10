use std::{borrow::Cow, collections::HashMap};

use testcontainers::{core::WaitFor, CopyDataSource, CopyToContainer, Image};
use super::postgres::Postgres;

const NAME: &str = "postgis/postgis";
const TAG: &str = "17-3.5";

#[derive(Debug, Clone, Default)]
pub struct Postgis(Postgres);

impl Postgis {
    /// Enables the Postgres instance to be used without authentication on host.
    /// For more information see the description of `POSTGRES_HOST_AUTH_METHOD` in official [docker image](https://hub.docker.com/_/postgres)
    pub fn with_host_auth(mut self) -> Self {
        Self(self.0.with_host_auth())
    }

    /// Sets the db name for the Postgres instance.
    pub fn with_db_name(mut self, db_name: &str) -> Self {
        Self(self.0.with_db_name(db_name))
    }

    /// Sets the user for the Postgres instance.
    pub fn with_user(mut self, user: &str) -> Self {
        Self(self.0.with_user(user))
    }

    /// Sets the password for the Postgres instance.
    pub fn with_password(mut self, password: &str) -> Self {
        Self(self.0.with_password(password))
    }

    /// Registers sql to be executed automatically when the container starts.
    /// Can be called multiple times to add (not override) scripts.
    ///
    /// # Example
    ///
    /// ```
    /// # use testcontainers_modules::postgres::Postgres;
    /// let postgres_image = Postgres::default().with_init_sql(
    ///     "CREATE EXTENSION IF NOT EXISTS hstore;"
    ///         .to_string()
    ///         .into_bytes(),
    /// );
    /// ```
    ///
    /// ```rust,ignore
    /// # use testcontainers_modules::postgres::Postgres;
    /// let postgres_image = Postgres::default()
    ///                                .with_init_sql(include_str!("path_to_init.sql").to_string().into_bytes());
    /// ```
    pub fn with_init_sql(mut self, init_sql: impl Into<CopyDataSource>) -> Self {
        Self(self.0.with_init_sql(init_sql))
    }

    /// Enables [the fsync-setting](https://www.postgresql.org/docs/current/runtime-config-wal.html#GUC-FSYNC) for the Postgres instance.
    pub fn with_fsync_enabled(mut self) -> Self {
        Self(self.0.with_fsync_enabled())
    }
}

impl Image for Postgis {
    fn name(&self) -> &str {
        NAME
    }

    fn tag(&self) -> &str {
        TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        self.0.ready_conditions()
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        self.0.env_vars()
    }

    fn copy_to_sources(&self) -> impl IntoIterator<Item = &CopyToContainer> {
        self.0.copy_to_sources()
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<std::borrow::Cow<'_, str>>> {
        self.0.cmd()
    }
}
