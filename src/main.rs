#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use core as _core;
use std::ptr::NonNull;

#[macro_use]
extern crate structopt;
extern crate libloading as ll; 
use structopt::StructOpt;
use std::path::PathBuf; 
use std::os::raw::c_void;
use std::mem::{forget};
use std::ptr::null_mut;
use std::ops::Deref;

pub trait Interface {
    // Returns the IID of the Interface
    fn uuidof() -> pluginterfaces::TUID;
}

#[macro_use]
mod macros;
mod pluginterfaces;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli-host")]
struct Opt {
    #[structopt(short= "p", long = "path")]
    #[structopt(parse(from_os_str))]
    path : PathBuf
}

fn main() {
    let opt = Opt::from_args();
    let mut lib_path = opt.path.clone(); 

    lib_path.push("Contents");
    lib_path.push("x86_64-linux"); //todo: cross platform version
    lib_path.push(
        opt.path.as_path()
           .file_stem()
           .unwrap()
           .to_str()
           .unwrap());

    let p = lib_path.as_path();
    let lib = ll::Library::new(lib_path
        .as_path())
        .expect("no such library");

    use pluginterfaces::*;
    unsafe {
        let GetPluginFactory : ll::Symbol<pluginterfaces::GetFactoryProc> = lib
            .get(b"GetPluginFactory")
            .expect("No Plugin Factory Defined");

        let factory  = VstPtr::from_raw(GetPluginFactory());
        let num_classes = (*factory).countClasses();
        
        for i in 0..num_classes {
            let mut info = PClassInfo::default(); 
            (*factory).getClassInfo(i, &mut info as *mut _);
            println!("{:?}", info);
        }

        let mut ptr = null_mut();
        let u       = factory.as_unknown();
        let next    = factory
            .as_unknown()
            .queryInterface(&IPluginFactory2::uuidof() as *const i8, &mut ptr as *mut *mut _);
        
        println!("{}\n{:?}", next, ptr);
    }
}
