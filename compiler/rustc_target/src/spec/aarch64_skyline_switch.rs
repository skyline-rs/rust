use super::{LinkerFlavor, LldFlavor, PanicStrategy, RelroLevel, Target, TargetOptions};

// skyline has custom linker requirements.
//const LINKER_SCRIPT: &str = include_str!("./aarch64_skyline_switch_linker_script.ld");

pub fn target() -> Target {
    let mut opts = TargetOptions {
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".to_owned()),
        os: "switch".to_string(),
        max_atomic_width: Some(128),
        panic_strategy: PanicStrategy::Abort,
        position_independent_executables: true,
        unsupported_abis: super::arm_base::unsupported_abis(),
        crt_static_default: false,
        crt_static_respected: false,
        dynamic_linking: true,
        executables: true,
        has_elf_tls: false,
        has_rpath: false,
        relro_level: RelroLevel::Off,
        //link_script: Some(LINKER_SCRIPT.to_string()),
        ..Default::default()
    };

    opts.pre_link_args.insert(
        LinkerFlavor::Lld(LldFlavor::Ld),
        vec![
            "-init=__custom_init".to_string(),
            "-fini=__custom_fini".to_string(),
            "--export-dynamic".to_string(),
        ],
    );

    opts.post_link_args.insert(
        LinkerFlavor::Lld(LldFlavor::Ld),
        vec!["--no-gc-sections".to_string(), "--eh-frame-hdr".to_string()],
    );

    Target {
        llvm_target: "aarch64-unknown-none".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options: opts,
    }
}
