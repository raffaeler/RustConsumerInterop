extern crate fs_extra;
//use fs_extra::copy_items;
//use std::{env, path::PathBuf};
//use fs_extra::dir::copy;

// fn combine(args: &[&str]) -> PathBuf {
//     let mut p = PathBuf::new();
//     for arg in args {
//         let seg = PathBuf::from(arg);
//         p.push(seg);
//     }

//     return p;
// }

// fn pushv(vec : &mut Vec<&str>, path : &PathBuf) {
//     let str = path.to_str();
//     vec.push(str.unwrap());
// }

fn main() {
/*
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    //let path = env::current_dir().unwrap();
    //let out_dir = env::var("OUT_DIR").unwrap();
    //println!("cargo:{}", out_dir);
    let bpath = env::current_dir().unwrap().join("..").canonicalize().unwrap();
    let root =bpath.to_str().unwrap();
    //println!("cargo:{}", bpath.display());

    //println!("cargo:{}", combine(&[root, "a", "b"]).display());
    let mut options = fs_extra::dir::CopyOptions::new(); //Initialize default values for CopyOptions
    options.overwrite = true;

    let mut from_paths = Vec::new();
    let s1 = &combine(&[root, "CalcComponent", "bin","x64","Debug","net5.0-windows10.0.19041.0","CalcComponent.winmd"]).to_str().unwrap();
    from_paths.push(s1);
    let s2 = &combine(&[root, "ChronoComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "ChronoComponent.winmd"]).to_str().unwrap();
    // pushv(&mut from_paths, &combine(&[root, "CalcComponent", "bin","x64","Debug","net5.0-windows10.0.19041.0","CalcComponent.winmd"]));
    // pushv(&mut from_paths, &combine(&[root, "ChronoComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "ChronoComponent.winmd"]));
    let res1 = fs_extra::copy_items(&from_paths, ".windows/winmd", &options);
    println!("cargo:{:?}", res1.err());

    // from_paths.clear();
    // pushv(&mut from_paths, &combine(&[root, "ChronoComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "ChronoComponent.dll"]));
    // pushv(&mut from_paths, &combine(&[root, "CalcComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "CalcComponent.dll"]));
    // pushv(&mut from_paths, &combine(&[root, "CalcComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "Microsoft.Windows.SDK.NET.dll"]));
    // pushv(&mut from_paths, &combine(&[root, "CalcComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "WinRT.Host.dll"]));
    // pushv(&mut from_paths, &combine(&[root, "CalcComponent", "bin", "x64", "Debug", "net5.0-windows10.0.19041.0", "WinRT.Host.Shim.dll"]));
    // pushv(&mut from_paths, &combine(&[root, "CalcComponent", "bin", "x64", "", "/net5.0-windows10.0.19041.0/WinRT.Runtime.dll"]));
    // let _res2 = fs_extra::copy_items(&from_paths, ".windows/x64", &options);

    // from_paths.clear();
    // pushv(&mut from_paths, &combine(&[root, "src/WinRT.Host.runtimeconfig.json"]));
    // pushv(&mut from_paths, &combine(&[root, "src/RustClientConsole.exe.manifest"]));
    // let _res3 = fs_extra::copy_items(&from_paths, "target/", &options);
*/

    windows::build!(Component1::Class1, Component2::Class2,);
}
