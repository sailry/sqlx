use std::{
    iter::{Extend, IntoIterator},
    sync::Arc,
};

use crate::MySqlColumn;

#[derive(Debug, Default)]
pub struct MySqlQueryResult {
    pub(super) rows_affected: u64,
    pub(super) last_insert_id: u64,
    pub(super) columns: Arc<Vec<MySqlColumn>>,
}

impl MySqlQueryResult {
    /// Returns the columns of this completed result set, including a result set
    /// with zero rows. Commands without a result set return an empty slice.
    pub fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    pub fn last_insert_id(&self) -> u64 {
        self.last_insert_id
    }

    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

impl Extend<MySqlQueryResult> for MySqlQueryResult {
    fn extend<T: IntoIterator<Item = MySqlQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
            self.last_insert_id = elem.last_insert_id;
            self.columns = elem.columns;
        }
    }
}
#[cfg(feature = "any")]
/// This conversion attempts to save last_insert_id by converting to i64.
impl From<MySqlQueryResult> for sqlx_core::any::AnyQueryResult {
    fn from(done: MySqlQueryResult) -> Self {
        sqlx_core::any::AnyQueryResult {
            rows_affected: done.rows_affected(),
            last_insert_id: done.last_insert_id().try_into().ok(),
        }
    }
}
