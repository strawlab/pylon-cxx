fn main() {
    println!("cargo:rerun-if-env-changed=PYLON_VERSION");
    println!("cargo:rerun-if-env-changed=PYLON_ROOT");
    println!("cargo:rerun-if-env-changed=PYLON_DEV_DIR");

    let pylon_major_version: Option<u8> =
        std::env::var_os("PYLON_VERSION").map(|s| s.into_string().unwrap().parse::<u8>().unwrap());

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/catcher.h");
    println!("cargo:rerun-if-changed=include/pylon-cxx-rs.h");
    println!("cargo:rerun-if-changed=src/pylon-cxx-rs.cc");

    let mut build = cxx_build::bridge("src/lib.rs");

    build
        .file("src/pylon-cxx-rs.cc")
        .warnings(false)
        .cpp(true)
        .include("include".to_string());

    #[cfg(target_os = "linux")]
    {
        let pylon_root = match std::env::var("PYLON_ROOT") {
            Ok(val) => val,
            Err(_) => match pylon_major_version {
                Some(5) => "/opt/pylon5",
                Some(6) | None => "/opt/pylon",
                Some(version) => panic!("unsupported pylon version: {}", version),
            }
            .into(),
        };

        let expected_major_version = match pylon_root.as_str() {
            "/opt/pylon5" => Some(5),
            "/opt/pylon" => Some(6),
            _ => None,
        };

        let pylon_major_version = match (expected_major_version, pylon_major_version) {
            (Some(expected_major_version), Some(actual_major_version)) => {
                assert_eq!(expected_major_version, actual_major_version);
                actual_major_version
            }
            (Some(v), None) | (None, Some(v)) => v,
            (None, None) => 6,
        };

        let pylon_root = std::path::PathBuf::from(pylon_root);

        let include1 = pylon_root.join("include");

        build.flag("-std=c++14").include(&include1);

        let mut lib_dir = pylon_root.clone();
        if pylon_major_version == 5 {
            lib_dir.push("lib64");
        } else {
            lib_dir.push("lib");
        }

        let dir_str = lib_dir.to_str().unwrap();

        println!("cargo:rustc-link-search=native={}", dir_str);
        println!("cargo:rustc-link-lib=pylonc");

        // The Basler docs want the rest of these libraries to be automatically
        // found using rpath linker args, but sending options to the linker in rust
        // requires the unstable link_args feature. So we specify them manually.
        // See https://github.com/rust-lang/cargo/issues/5077
        println!("cargo:rustc-link-lib=pylonbase");
        println!("cargo:rustc-link-lib=pylonutility");
        println!("cargo:rustc-link-lib=gxapi");

        if pylon_major_version == 5 {
            enum PylonVersion {
                V5_0,
                V5_1,
                V5_2,
                Unknown,
            }

            let mut so_file_for_5_2 = lib_dir.clone();
            so_file_for_5_2.push("libpylon_TL_usb-5.2.0.so");

            let mut so_file_for_5_1 = lib_dir.clone();
            so_file_for_5_1.push("libGenApi_gcc_v3_1_Basler_pylon_v5_1");
            so_file_for_5_1.set_extension("so");

            eprint!(
                "# pylon build: checking for file {}...",
                so_file_for_5_2.display()
            );
            let version = if so_file_for_5_2.exists() {
                eprintln!("found");
                PylonVersion::V5_2
            } else {
                eprintln!("not found");

                eprint!(
                    "# pylon build: checking for file {}...",
                    so_file_for_5_1.display()
                );
                if so_file_for_5_1.exists() {
                    eprintln!("found");
                    PylonVersion::V5_1
                } else {
                    eprintln!("not found");
                    let mut so_file_for_5_0 = lib_dir.clone();
                    so_file_for_5_0.push("libGenApi_gcc_v3_0_Basler_pylon_v5_0");
                    so_file_for_5_0.set_extension("so");
                    eprint!(
                        "# pylon build: checking for file {}...",
                        so_file_for_5_0.display()
                    );
                    if so_file_for_5_0.exists() {
                        eprintln!("found");
                        PylonVersion::V5_0
                    } else {
                        eprintln!("not found");
                        PylonVersion::Unknown
                    }
                }
            };

            match version {
                PylonVersion::V5_0 => {
                    println!("cargo:rustc-link-lib=GenApi_gcc_v3_0_Basler_pylon_v5_0");
                    println!("cargo:rustc-link-lib=GCBase_gcc_v3_0_Basler_pylon_v5_0");
                    println!("cargo:rustc-link-lib=Log_gcc_v3_0_Basler_pylon_v5_0");
                    println!("cargo:rustc-link-lib=MathParser_gcc_v3_0_Basler_pylon_v5_0");
                    println!("cargo:rustc-link-lib=XmlParser_gcc_v3_0_Basler_pylon_v5_0");
                    println!("cargo:rustc-link-lib=NodeMapData_gcc_v3_0_Basler_pylon_v5_0");
                }
                PylonVersion::V5_1 => {
                    println!("cargo:rustc-link-lib=GenApi_gcc_v3_1_Basler_pylon_v5_1");
                    println!("cargo:rustc-link-lib=GCBase_gcc_v3_1_Basler_pylon_v5_1");
                    println!("cargo:rustc-link-lib=Log_gcc_v3_1_Basler_pylon_v5_1");
                    println!("cargo:rustc-link-lib=MathParser_gcc_v3_1_Basler_pylon_v5_1");
                    println!("cargo:rustc-link-lib=XmlParser_gcc_v3_1_Basler_pylon_v5_1");
                    println!("cargo:rustc-link-lib=NodeMapData_gcc_v3_1_Basler_pylon_v5_1");
                }
                PylonVersion::V5_2 => {
                    println!("cargo:rustc-link-lib=GenApi_gcc_v3_1_Basler_pylon");
                    println!("cargo:rustc-link-lib=GCBase_gcc_v3_1_Basler_pylon");
                    println!("cargo:rustc-link-lib=Log_gcc_v3_1_Basler_pylon");
                    println!("cargo:rustc-link-lib=MathParser_gcc_v3_1_Basler_pylon");
                    println!("cargo:rustc-link-lib=XmlParser_gcc_v3_1_Basler_pylon");
                    println!("cargo:rustc-link-lib=NodeMapData_gcc_v3_1_Basler_pylon");
                }
                PylonVersion::Unknown => {
                    panic!("could not detect pylon library version");
                }
            }
        } else {
            assert_eq!(pylon_major_version, 6);

            // The following are for Pylon 6.1 and may need to be updated for other versions.
            println!("cargo:rustc-link-lib=GenApi_gcc_v3_1_Basler_pylon");
            println!("cargo:rustc-link-lib=GCBase_gcc_v3_1_Basler_pylon");
            println!("cargo:rustc-link-lib=Log_gcc_v3_1_Basler_pylon");
            println!("cargo:rustc-link-lib=MathParser_gcc_v3_1_Basler_pylon");
            println!("cargo:rustc-link-lib=XmlParser_gcc_v3_1_Basler_pylon");
            println!("cargo:rustc-link-lib=NodeMapData_gcc_v3_1_Basler_pylon");
        }
    }

    #[cfg(target_os = "macos")]
    {
        match pylon_major_version {
            Some(5) => {
                todo!()
            }
            Some(6) | None => {
                assert!(pylon_major_version == Some(6) || pylon_major_version.is_none());
                println!("cargo:rustc-link-search=framework=/Library/Frameworks/");
                println!("cargo:rustc-link-lib=framework=pylon");

                build
                    .flag("-std=c++14")
                    .include("/Library/Frameworks/pylon.framework/Headers/GenICam")
                    .include("/Library/Frameworks/pylon.framework/Headers")
                    .flag("-F/Library/Frameworks");
            }
            Some(version) => panic!("unsupported pylon version: {}", version),
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::path::PathBuf;

        let pylon_dev_dir = match std::env::var("PYLON_DEV_DIR") {
            Ok(val) => PathBuf::from(val),
            Err(_) => match pylon_major_version {
                Some(5) => PathBuf::from(r#"C:\Program Files\Basler\pylon 5\Development"#),
                Some(6) | None => PathBuf::from(r#"C:\Program Files\Basler\pylon 6\Development"#),
                Some(version) => panic!("unsupported pylon version: {}", version),
            },
        };

        let mut include_dir = pylon_dev_dir.clone();
        include_dir.push("include");

        let mut pylon_include_dir = include_dir.clone();
        pylon_include_dir.push("pylon");

        let mut lib_dir = pylon_dev_dir;
        lib_dir.push("lib");
        lib_dir.push("x64");

        println!("cargo:rustc-link-search={}", lib_dir.display());

        build.include(include_dir);
    }

    build.compile("pyloncxxrs-demo");
}
