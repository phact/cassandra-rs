use cql_bindgen::CassIterator as _CassIterator;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_column;
use cql_bindgen::CassRow as _CassRow;
use cql_bindgen::cass_row_get_column;
use cql_bindgen::cass_row_get_column_by_name;
use cql_bindgen::cass_iterator_from_row;
use cql_bindgen::CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::ffi::CString;
use std::iter::IntoIterator;
use std::iter;

use cql_ffi::value::CassValue;
use cql_ffi::error::CassError;
use cql_ffi::column::CassColumn;

pub struct CassRow(pub *const _CassRow);

impl Debug for CassRow {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for column in self {
            //println!("foo:{:?}",column);
            try!(write!(f, "{:?}\t", CassValue::new(column.0)));
        }
        Ok(())
    }
}

impl Display for CassRow {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for column in self {
            try!(write!(f, "{}\t", CassValue::new(column.0)));
        }
        Ok(())
    }
}

impl CassRow {
    pub fn get_column(&self, index: u64) -> Result<CassColumn, CassError> {
        unsafe {
            let col = cass_row_get_column(self.0, index);
            match col.is_null() {
                true => Err(CassError::build(CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS)),
                false => Ok(CassColumn(col)),
            }
        }
    }

    pub fn get_column_by_name<S>(&self, name: S) -> CassColumn
        where S: Into<String>
    {
        unsafe {
            let name = CString::new(name.into()).unwrap();
            println!("name: {:?}", name);
            println!("self: {:?}", self);
        //unimplemented!();
            CassColumn(cass_row_get_column_by_name(self.0, name.as_ptr()))
        }
    }
}

pub struct RowIterator(pub *mut _CassIterator);


impl Drop for RowIterator {
    fn drop(&mut self) {
        unsafe {
            cass_iterator_free(self.0)
        }
    }
}

impl iter::Iterator for RowIterator {

    type Item = CassColumn;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(CassColumn(cass_iterator_get_column(self.0))),
            }
        }
    }
}

impl<'a> Iterator for &'a RowIterator {

    type Item = CassColumn;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(CassColumn(cass_iterator_get_column(self.0))),
            }
        }
    }
}

impl Display for RowIterator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for item in self {
            try!(write!(f, "{}\t", CassValue::new(item.0)));
        }
        Ok(())
    }
}

impl IntoIterator for CassRow {

    type Item = CassColumn;
    type IntoIter = RowIterator;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            RowIterator(cass_iterator_from_row(self.0))
        }
    }
}

impl<'a> IntoIterator for &'a CassRow {
    type Item = CassColumn;
    type IntoIter = RowIterator;
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            RowIterator(cass_iterator_from_row(self.0))
        }
    }
}
