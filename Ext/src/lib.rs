#![allow(non_snake_case)]

use std::{os::raw::c_char, ffi::CStr, ffi::CString};
use redis::Commands;

pub fn get_c_string(c_s: *const c_char) -> String {
  let r_string: String = unsafe {
      CStr::from_ptr(c_s).to_string_lossy().into_owned()
  };

  return r_string;
}

#[no_mangle]
pub extern "C" fn createRedisConnection(host: *const c_char) -> *mut redis::Connection {
  let host = get_c_string(host);
  let client = redis::Client::open(host).unwrap();
  let connection = client.get_connection().unwrap();
  Box::into_raw(Box::new(connection))
}

#[no_mangle]
pub extern "C" fn setRedisKey(connection: *mut redis::Connection, key: *const c_char, value: *const c_char) {
  let key = get_c_string(key);
  let value = get_c_string(value);
  let connection = unsafe { &mut *connection };
  connection.set(key, value).unwrap()
}

#[no_mangle]
pub extern "C" fn getRedisKey(connection: *mut redis::Connection, key: *const c_char) -> *mut c_char {
  let key = get_c_string(key);
  let connection = unsafe { &mut *connection };
  let value: String = redis::cmd("GET").arg(key).query(connection).unwrap();
  CString::new(value).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn destroyRedisConnection(connection: *mut redis::Connection) {
  unsafe {
    Box::from_raw(connection);
  }
}
