extern crate libc;

use libc::AF_INET;
use libc::SOCK_RAW;
use libc::addrinfo;
use libc::getaddrinfo;
use libc::gai_strerror;

use std::env;
use std::ffi;
use std::io;
use std::io::Write;
use std::mem;
use std::process;
use std::ptr;
use std::str;

const IPPROTO_ICMP: libc::c_int = 1;

fn main() {
  if env::args().count() < 2 {
    writeln!(io::stderr(), "Usage: ping <host>");
    process::exit(1);
  }

  let mut addrinfo_hints: libc::addrinfo = unsafe {
    mem::zeroed::<addrinfo>()
  };
  addrinfo_hints.ai_family = AF_INET;
  addrinfo_hints.ai_socktype = SOCK_RAW;
  addrinfo_hints.ai_protocol = IPPROTO_ICMP;

  let host = env::args().nth(1).unwrap();
  let mut addrinfo_head: *mut addrinfo = ptr::null_mut();

  let host_c = ffi::CString::new(host).unwrap().as_ptr();
  unsafe {
    let error = getaddrinfo(host_c,
                            ptr::null(),
                            &addrinfo_hints,
                            &mut addrinfo_head);
    if error != 0 {
      let error_str = ffi::CStr::from_ptr(gai_strerror(error)).to_bytes();
      let error_str = str::from_utf8(error_str).unwrap();
      writeln!(io::stderr(), "Error: getaddrinfo: {}", error_str);
      process::exit(1);
    }
  }
}
