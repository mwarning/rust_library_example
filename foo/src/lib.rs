
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

struct Bar {
	data: Vec<u8>
}

impl Bar {
	fn new() -> Bar {
		Bar{data: vec![]}
	}
}

impl RandomAccessMethods for Bar {
  fn open(&mut self) -> Result<(), Error> {
  	Ok(())
  }

  /// Write bytes at an offset to the backend.
  fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
    println!("write(offset: {}, data: {:?})", offset, data);
    self.data = data.to_vec();
    Ok(())
  }

  /// Read a sequence of bytes at an offset from the backend.
  fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, Error> {
  	println!("read(offset: {}, length: {})", offset, length);
  	if self.data.len() == length {
  		Ok(self.data.to_owned())
  	} else {
  		Err(failure::err_msg("failure"))
  	}
  }

  /// Delete a sequence of bytes at an offset from the backend.
  fn del(&mut self, offset: usize, length: usize) -> Result<(), Error> {
  	println!("del(offset: {}, length: {})", offset, length);
  	Ok(())
  }
}

#[no_mangle]
pub extern "C"
fn dat_new() -> *mut RandomAccessMethods {
  Box::into_raw(Box::new(Bar::new()))
}

#[no_mangle]
pub extern "C"
fn dat_free(ptr: *mut RandomAccessMethods) {
  if ptr.is_null() { return }
  unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern "C"
fn dat_write(ptr: *mut RandomAccessMethods, offset: usize, length: usize, array: *const u8) -> i32 {
	println!("dat_write()");
	let array_slice = unsafe { std::slice::from_raw_parts(array, length) };
	let obj = unsafe { &mut Box::from_raw(ptr) };
	match obj.write(offset, array_slice) {
		Ok(_) => 0,
 		Err(_) => 1 
	}
}


#[no_mangle]
pub extern "C"
fn dat_read(ptr: *mut RandomAccessMethods, offset: usize, length: usize, array: *mut u8) -> i32 {
	println!("dat_read()");
	let array_slice = unsafe { std::slice::from_raw_parts_mut(array, length) };
	let obj = unsafe { &mut Box::from_raw(ptr) };
	if let Ok(vec) = obj.read(offset, length) {
		// Copy from vec into array
		array_slice.copy_from_slice(&vec[..]);
		return 0;
	}
	return 1;
}
