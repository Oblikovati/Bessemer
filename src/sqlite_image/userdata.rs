use super::{SqliteImage, SqliteRows, connection, query};
use crate::Result;

const USERDATA_SQL: &str = concat!(
    "SELECT use_objectid,use_objectno,use_data0,use_data1,use_data2,use_data3,",
    "use_data4,use_data5,use_data6,use_data7,use_data8,use_data9,use_data10,",
    "use_data11,use_data12,use_data13,use_data14,use_data15,use_data16,",
    "use_data17,use_data18,use_data19 FROM tew_userdata ",
    "WHERE use_objectid=?1 AND use_objectno=?2"
);

impl SqliteImage {
    pub(crate) fn electrical_userdata_rows(
        &self,
        object_id: i32,
        object_no: i32,
    ) -> Result<SqliteRows> {
        let database = connection::open_image(&self.original, self.limits)?;
        query::read_bound_rows(
            &database,
            USERDATA_SQL,
            [object_id, object_no],
            "tew_userdata",
            self.limits,
            &mut 0,
        )
    }
}
