#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use core as _core;

/*
#[macro_use]
mod macros;
mod pluginterfaces;
*/
#[macro_use]
extern crate structopt;
extern crate libloading as ll;

use winit::{Event, WindowEvent, WindowBuilder, EventsLoop};
use winit::dpi::LogicalSize;
use std::mem::transmute;
use x11::xlib::XInternAtom;
use structopt::StructOpt;
use std::path::PathBuf; 
use std::os::raw::c_void;
use std::mem::{forget};
use std::ptr::null_mut;
use std::ops::Deref;
use std::sync::mpsc::TrySendError::Full;
use vst3_interfaces::*;
use vst3_interfaces::vst::SymbolicSampleSizes::{kSample32, kSample64};

mod host_impl;
use host_impl::*;
use winit::os::unix::WindowExt;



#[derive(StructOpt, Debug)]
#[structopt(name = "cli-host")]
struct Opt {
    #[structopt(short= "p", long = "path")]
    #[structopt(parse(from_os_str))]
    path : PathBuf
}

fn open (controller : Editor) {
    let view = controller.create_view();
    let size = view.get_size().unwrap();

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(LogicalSize::new(size.right as f64,size.bottom as f64))
        .with_title("VST3 Hosted in Rust!")
        .build(&events_loop).unwrap();

    if let Some (display) = window.get_xlib_display() {
        let atom = unsafe {
            XInternAtom (
                display as *mut _,
                transmute("_XEMBED_INFO\0".as_ptr()),
                1)
        };
        if atom == 0 { panic!("XEMBED_INFO does not exist"); }
       // view.set_frame(null_mut());
        view.attached (display)
            .expect("failed to attach to platform handle");

        loop {
            events_loop.poll_events(|event| {
                match event { _ => () }
            });
        }
    }
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

    let mut instances: Vec<(VstPtr<FUnknown>, PClassInfo)> = Vec::new();
    let GetPluginFactory: ll::Symbol<GetFactoryProc> = unsafe {
        lib
            .get(b"GetPluginFactory")
            .expect("No Plugin Factory Defined")
    };

    let factory = unsafe { VstPtr::from_raw(GetPluginFactory()) };
    let num_classes = unsafe { (*factory).countClasses() };

    for i in 0..num_classes {
        let mut info = PClassInfo::default();
        let mut ptr: *mut c_void = null_mut();

        unsafe {
            (*factory).getClassInfo(i, &mut info as *mut _);
            (*factory).createInstance(
                info.cid.as_ptr(),
                FUnknown::iid().as_ptr(),
                &mut ptr as *mut _);
            instances.push((VstPtr::from_raw(ptr as *mut _), info));
        }
    }

    let mut control = None;
    for inst in instances.iter() {
        println!("------------");
        println!("{:?}", inst.1);
        if let Ok(processor) = inst.0.cast::<IAudioProcessor>() {
            let proc = Processor::new(processor);
            println!("\taudio processor detected");
            if let Ok(_) = proc.set_mono() {
                println!("\tmono supported");
            }
            if let Ok(_) = proc.set_stereo() {
                println!("\tstereo supported");
            }
            println!("\ttail samples: {}\n\tlatency samples: {}", proc.tail_samples(), proc.latency_samples());

            if proc.supports32() { println!("\t32 bit audio supported"); }
            if proc.supports64() { println!("\t64 bit audio supported"); }
            if let Ok(_) = proc.setup_processing(
                ProcessMode::Offline,
                SampleSize::F32,
                512,
                48000.0) {
                println!("\tready for offline processing");
            }
        }

        if let Ok(controller) = inst.0.cast::<IEditController>() {
            println!("\tedit controller detected");
            control = Some(Editor::new(controller));
        }
    }

    let control = control.unwrap();
    open(control);
}
