use std::fmt::Formatter;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::mem;
use std::ffi::CStr;
use std::str;

use cql_bindgen::CassUuid as _CassUuid;
use cql_bindgen::CassUuidGen as _CassUuidGen;
use cql_bindgen::cass_uuid_gen_new;
use cql_bindgen::cass_uuid_gen_free;
use cql_bindgen::cass_uuid_gen_time;
use cql_bindgen::cass_uuid_gen_new_with_node;
use cql_bindgen::cass_uuid_gen_random;
use cql_bindgen::cass_uuid_gen_from_time;
use cql_bindgen::cass_uuid_min_from_time;
use cql_bindgen::cass_uuid_max_from_time;
use cql_bindgen::cass_uuid_timestamp;
use cql_bindgen::cass_uuid_version;
use cql_bindgen::cass_uuid_string;
//use cql_bindgen::raw2utf8;
//use cql_bindgen::cass_uuid_from_string;

//use cql_ffi::error::CassError;

const CASS_UUID_STRING_LENGTH:usize = 37;


#[derive(Copy,Clone)]
pub struct CassUuid(pub _CassUuid);

impl ::std::default::Default for CassUuid {
    fn default() -> CassUuid {
        unsafe {
            ::std::mem::zeroed()
        }
    }
}

pub struct CassUuidGen(pub *mut _CassUuidGen);

impl Drop for CassUuidGen {
    fn drop(&mut self) {
        unsafe {
            cass_uuid_gen_free(self.0)
        }
    }
}

impl Debug for CassUuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Display for CassUuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl CassUuid {
    pub fn min_from_time(&mut self, time: u64) {
        unsafe {
            cass_uuid_min_from_time(time, &mut self.0)
        }
    }

    pub fn max_from_time(&mut self, time: u64) {
        unsafe {
            cass_uuid_max_from_time(time, &mut self.0)
        }
    }

    pub fn timestamp(&self) -> u64 {
        unsafe {
            cass_uuid_timestamp(self.0)
        }
    }

    pub fn version(&self) -> u8 {
        unsafe {
            cass_uuid_version(self.0)
        }
    }

    //FIXME
    pub fn to_string(&self) -> String {
        unsafe {
            let mut time_str: [i8; CASS_UUID_STRING_LENGTH] = [0;CASS_UUID_STRING_LENGTH];

            cass_uuid_string(self.0, time_str[..].as_mut_ptr());
            let mut output: i8 = mem::zeroed();
            cass_uuid_string(self.0, &mut output);
            let slice = CStr::from_ptr(&output);
            str::from_utf8(slice.to_bytes()).unwrap().to_owned()
        }
    }

//    pub unsafe fn from_string(&mut self, str: *const c_char) -> Result<(),CassError> {
//        CassError::build(cass_uuid_from_string(str,&mut self.0))
//    }
}

impl CassUuidGen {
    pub fn new() -> Self {
        unsafe {
            CassUuidGen(cass_uuid_gen_new())
        }
    }

    pub fn new_with_node(node: u64) -> CassUuidGen {
        unsafe {
            CassUuidGen(cass_uuid_gen_new_with_node(node))
        }
    }

    pub fn get_time(&self) -> CassUuid {
        unsafe {
            let mut output: _CassUuid = mem::zeroed();
            cass_uuid_gen_time(self.0, &mut output);
            CassUuid(output)
        }
    }

    pub fn fill_random(&self, mut output: CassUuid) {
        unsafe {
            cass_uuid_gen_random(self.0, &mut output.0)
        }
    }

    pub fn random(&self) -> CassUuid {
        unsafe {
            let mut output: _CassUuid = mem::zeroed();
            cass_uuid_gen_random(self.0, &mut output);
            CassUuid(output)
        }
    }

    pub fn with_time(&self, timestamp: u64, mut output: CassUuid) {
        unsafe {
            cass_uuid_gen_from_time(self.0, timestamp, &mut output.0)
        }
    }
}
