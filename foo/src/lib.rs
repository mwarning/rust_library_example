
extern crate failure;

use failure::Error;


pub trait RandomAccessMethods {
  /// Open the backend.
  fn open(&mut self) -> Result<(), Error>;

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error>;

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error>;

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error>;
}


pub struct Bar {
  is_open: bool,
  data: Vec<u8>,
}

impl Bar {
  fn new() -> Bar {
    Bar{is_open: false, data: vec![]}
  }
}

impl RandomAccessMethods for Bar {
  fn open(&mut self) -> Result<(), Error> {
    if self.is_open {
      Err(failure::err_msg("already open"))
    } else {
      self.is_open = true;
      Ok(())
    }
  }

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
    if self.is_open {
      println!("write(self: {:p}, offset: {}, data: {:?})", self, offset, data);
      self.data = data.to_vec();
      Ok(())
    } else {
      Err(failure::err_msg("closed"))
    }
  }

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
    println!("read(self: {:p}, offset: {}, length: {})", self, offset, length);
    if self.is_open {
      if self.data.len() == length {
        Ok(self.data.to_owned())
      } else {
        Err(failure::err_msg("failure"))
      }
    } else {
      Err(failure::err_msg("closed"))
    }
  }

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error> {
    println!("del(self: {:p}, offset: {}, length: {})", self, offset, length);
    Err(failure::err_msg("not implemented"))
  }
}

#[no_mangle]
pub extern "C"
fn dat_new() -> *mut Bar {
  Box::into_raw(Box::new(Bar::new()))
}

#[no_mangle]
pub unsafe extern "C"
fn dat_free(ptr: *mut Bar) {
  drop(Box::from_raw(ptr));
}

#[no_mangle]
pub unsafe extern "C"
fn dat_open(ptr: *mut Bar) -> i32 {
  let obj =  &mut *ptr;
  match obj.open() {
    Ok(_) => 0,
    Err(_) => 1
  }
}

#[no_mangle]
pub unsafe extern "C"
fn dat_write(ptr: *mut Bar, offset: usize, length: usize, array: *const u8) -> i32 {
  let obj =  &mut *ptr;
  let array_slice = std::slice::from_raw_parts(array, length);

  match obj.write(offset, array_slice) {
    Ok(_) => 0,
    Err(_) => 1
  }
}

#[no_mangle]
pub unsafe extern "C"
fn dat_read(ptr: *mut Bar, offset: usize, length: usize, array: *mut u8) -> i32 {
  let obj = &mut *ptr;
  let array_slice = std::slice::from_raw_parts_mut(array, length);

  if let Ok(vec) = obj.read(offset, length) {
    // Copy from vec into array
    array_slice.copy_from_slice(&vec[..]);
    0
  } else {
    1
  }
}
