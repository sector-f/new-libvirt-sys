extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=libvirt");

    // # include <libvirt/libvirt-common.h>
    // # include <libvirt/libvirt-host.h>
    // # include <libvirt/libvirt-domain.h>
    // # include <libvirt/libvirt-domain-snapshot.h>
    // # include <libvirt/libvirt-event.h>
    // # include <libvirt/libvirt-interface.h>
    // # include <libvirt/libvirt-network.h>
    // # include <libvirt/libvirt-nodedev.h>
    // # include <libvirt/libvirt-nwfilter.h>
    // # include <libvirt/libvirt-secret.h>
    // # include <libvirt/libvirt-storage.h>
    // # include <libvirt/libvirt-stream.h>

        let bindings = bindgen::Builder::default()
            .header_contents("libvirt.h", "#include <libvirt/libvirt.h>\n#include <libvirt/virterror.h>")
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}

