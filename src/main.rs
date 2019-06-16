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
use std::sync::mpsc::TrySendError::Full;

pub trait Interface {
    // Returns the IID of the Interface
    fn iid() -> pluginterfaces::TUID;
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
    lib_path.set_extension("so");

    let lib = ll::Library::new(lib_path
        .as_path())
        .expect("no such library");

    use pluginterfaces::*;
    unsafe {
        let GetPluginFactory : ll::Symbol<GetFactoryProc> = lib
            .get(b"GetPluginFactory")
            .expect("No Plugin Factory Defined");

        let factory  = VstPtr::from_raw(GetPluginFactory());
        let num_classes = (*factory).countClasses();
        let mut instances : Vec<VstPtr<FUnknown>> = Vec::new();

        for i in 0..num_classes {
            let mut info = PClassInfo::default(); 
            (*factory).getClassInfo(i, &mut info as *mut _);
            println!("{:?}", info);
            let mut ptr:* mut c_void = null_mut();
            (*factory).createInstance (info.cid.as_ptr(), FUnknown::iid().as_ptr(), &mut ptr as *mut _);
            instances.push(VstPtr::from_raw(ptr as *mut _));
        }

        for inst in instances.iter() {
            if let Ok(_proc) = inst.cast::<IAudioProcessor>() {
                println!("audio processor created");
            }
            if let Ok(_controller) = inst.cast::<IEditController>() {
                println!("edit controller created");
            }
        }

    }
}
