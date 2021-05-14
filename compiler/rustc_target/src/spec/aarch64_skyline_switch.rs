use crate::abi::Endian;
use super::{LinkerFlavor, LinkArgs, LldFlavor, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), vec![
        "-Tlink.ld".to_string(),
        "-init=__custom_init".to_string(),
        "-fini=__custom_fini".to_string(),
        "--export-dynamic".to_string()
    ]);
    let mut post_link_args = LinkArgs::new();
    post_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), vec![
        "--no-gc-sections".to_string(),
        "--eh-frame-hdr".to_string()
    ]);

    let options = TargetOptions {
        crt_static_default: false,
        crt_static_respected: false,
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".to_owned()),
        features: "+strict-align,+neon,+fp-armv8".to_string(),
        executables: true,
        relocation_model: RelocModel::Pic,
        disable_redzone: true,
        //linker_is_gnu: true,
        max_atomic_width: Some(128),
        panic_strategy: PanicStrategy::Abort,
        unsupported_abis: super::arm_base::unsupported_abis(),
        endian: Endian::Little,
        dynamic_linking: true,
        //dll_prefix: "".to_string(),
        os: "switch".to_string(),
        pre_link_args,
        post_link_args,
        ..Default::default()
    };
    Target {
        llvm_target: "aarch64-unknown-none".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options,
    }
}
