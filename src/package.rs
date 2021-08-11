use serde_derive::{Deserialize, Serialize};
use toml::Value;

use crate::license::NixLicense;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Sources {
    #[serde(rename = "vsmarketplace")]
    VSCodeMarketPlace,
    #[serde(rename = "openvsx")]
    OpenVSX,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub src: Sources,
}

impl Package {
    pub fn get_packages(toml_file_as_string: String) -> Vec<Package> {
        let mut packages: Vec<Package> = Vec::new();
        let mut tml = toml_file_as_string.parse::<Value>().unwrap();

        if let Some(table) = tml.as_table_mut() {
            for (k, v) in table {
                if v.is_table() {
                    if let Some(t) = v.as_table_mut() {
                        t.insert(String::from("name"), Value::String(k.to_string()));
                    }
                    let package: Package = toml::from_str(&v.to_string()).unwrap();
                    packages.push(package)
                }
            }
        }
        packages
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum PackageKind {
    VscodeExtension {
        publisher: String,
        extension_name: String,
    },
    OpenVSX {
        publisher: String,
        extension_name: String,
    },
    Other,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct NixPackage {
    pub kind: PackageKind,
    pub name: String,
    pub pname: String,
    pub src: String,
    pub version: String,
    pub sha256: String,
    pub meta: NixPackageMeta,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct NixPackageMeta {
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub branch: Option<String>,
    pub homepage: Option<String>,
    pub download_page: Option<String>,
    pub changelog: Option<Vec<String>>,
    pub license: Option<Vec<NixLicense>>,
    pub priority: Option<String>,
    pub maintainers: Option<String>,
    pub platforms: Option<NixPlatforms>,
    pub tests: Option<String>,
    pub timeout: Option<u64>,
    pub hydra_platforms: Option<String>,
    pub broken: Option<bool>,
    pub update_walker: Option<bool>,
}

impl Default for NixPackageMeta {
    fn default() -> Self {
        NixPackageMeta {
            description: None,
            long_description: None,
            branch: None,
            homepage: None,
            download_page: None,
            changelog: None,
            license: None,
            priority: None,
            maintainers: Some(String::from("th3whit3wolf")),
            platforms: Some(NixPlatforms::None),
            tests: None,
            timeout: None,
            hydra_platforms: None,
            broken: None,
            update_walker: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum NixPlatforms {
    Aarch64,
    Aarch64Darwin,
    Aarch64Genode,
    Aarch64Linux,
    Aarch64Netbsd,
    Aarch64None,
    All,
    Arm,
    ArmNone,
    Armv5telLinux,
    Armv6lLinux,
    Armv6lNetbsd,
    Armv6lNone,
    Armv7aDarwin,
    Armv7aLinux,
    Armv7aNetbsd,
    Armv7lLinux,
    Armv7lNetbsd,
    AvrNone,
    BigEndian,
    Cygwin,
    Darwin,
    Embedded,
    FreeBSD,
    Genode,
    Gnu,
    I686,
    I686Cygwin,
    I686Darwin,
    I686Freebsd,
    I686Genode,
    I686Linux,
    I686Netbsd,
    I686None,
    I686Openbsd,
    I686Windows,
    Illumos,
    Js,
    JsGhcjs,
    Linux,
    LittleEndian,
    M68k,
    M68kLinux,
    M68kNetbsd,
    M68kNone,
    MesaPlatforms,
    Mips,
    MipselLinux,
    MipselNetbsd,
    Mmix,
    MmixMmixware,
    Msp430None,
    Netbsd,
    None,
    Openbsd,
    Or1k,
    Or1kNone,
    Powerpc64leLinux,
    Powerpc64Linux,
    PowerpcNetbsd,
    PowerpcNone,
    Redox,
    Riscv,
    Riscv32Linux,
    Riscv32Netbsd,
    Riscv32None,
    Riscv64Linux,
    Riscv64Netbsd,
    Riscv64None,
    S390,
    S390Linux,
    S390None,
    Unix,
    Vc4,
    Vc4None,
    Wasi,
    Wasm32Wasi,
    Wasm64Wasi,
    Windows,
    X86,
    X86_64,
    X86_64Cygwin,
    X86_64Darwin,
    X86_64Freebsd,
    X86_64Genode,
    X86_64Linux,
    X86_64Netbsd,
    X86_64None,
    X86_64Openbsd,
    X86_64Redox,
    X86_64Solaris,
    X86_64Windows,
}

impl NixPlatforms {
    fn to_nix_meta(&self) -> String {
        match self {
            NixPlatforms::Aarch64 => String::from("lib.platforms.aarch64"),
            NixPlatforms::Aarch64Darwin => String::from("lib.platforms.aarch64-darwin"),
            NixPlatforms::Aarch64Genode => String::from("lib.platforms.aarch64-genode"),
            NixPlatforms::Aarch64Linux => String::from("lib.platforms.aarch64-linux"),
            NixPlatforms::Aarch64Netbsd => String::from("lib.platforms.aarch64-netbsd"),
            NixPlatforms::Aarch64None => String::from("lib.platforms.aarch64-none"),
            NixPlatforms::All => String::from("lib.platforms.all"),
            NixPlatforms::Arm => String::from("lib.platforms.arm"),
            NixPlatforms::ArmNone => String::from("lib.platforms.arm-none"),
            NixPlatforms::Armv5telLinux => String::from("lib.platforms.armv5tel-linux"),
            NixPlatforms::Armv6lLinux => String::from("lib.platforms.armv6l-linux"),
            NixPlatforms::Armv6lNetbsd => String::from("lib.platforms.armv6l-netbsd"),
            NixPlatforms::Armv6lNone => String::from("lib.platforms.armv6l-none"),
            NixPlatforms::Armv7aDarwin => String::from("lib.platforms.armv7a-darwin"),
            NixPlatforms::Armv7aLinux => String::from("lib.platforms.armv7a-linux"),
            NixPlatforms::Armv7aNetbsd => String::from("lib.platforms.armv7a-netbsd"),
            NixPlatforms::Armv7lLinux => String::from("lib.platforms.armv7-linux"),
            NixPlatforms::Armv7lNetbsd => String::from("lib.platforms.armv7-netbsd"),
            NixPlatforms::AvrNone => String::from("lib.platforms.avr-none"),
            NixPlatforms::BigEndian => String::from("lib.platforms.bigEndian"),
            NixPlatforms::Cygwin => String::from("lib.platforms.cygwin"),
            NixPlatforms::Darwin => String::from("lib.platforms.darwin"),
            NixPlatforms::Embedded => String::from("lib.platforms.embedded"),
            NixPlatforms::FreeBSD => String::from("lib.platforms.freeBSD"),
            NixPlatforms::Genode => String::from("lib.platforms.genode"),
            NixPlatforms::Gnu => String::from("lib.platforms.gnu"),
            NixPlatforms::I686 => String::from("lib.platforms.i686"),
            NixPlatforms::I686Cygwin => String::from("lib.platforms.i686-cygwin"),
            NixPlatforms::I686Darwin => String::from("lib.platforms.i686-darwin"),
            NixPlatforms::I686Freebsd => String::from("lib.platforms.i686-freeBSD"),
            NixPlatforms::I686Genode => String::from("lib.platforms.i686-genode"),
            NixPlatforms::I686Linux => String::from("lib.platforms.i686-linux"),
            NixPlatforms::I686Netbsd => String::from("lib.platforms.i686-netbsd"),
            NixPlatforms::I686None => String::from("lib.platforms.i686-none"),
            NixPlatforms::I686Openbsd => String::from("lib.platforms.i686-openbsd"),
            NixPlatforms::I686Windows => String::from("lib.platforms.i686-windows"),
            NixPlatforms::Illumos => String::from("lib.platforms.illumos"),
            NixPlatforms::Js => String::from("lib.platforms.js"),
            NixPlatforms::JsGhcjs => String::from("lib.platforms.js-ghcjs"),
            NixPlatforms::Linux => String::from("lib.platforms.linux"),
            NixPlatforms::LittleEndian => String::from("lib.platforms.littleEndian"),
            NixPlatforms::M68k => String::from("lib.platforms.m68k"),
            NixPlatforms::M68kLinux => String::from("lib.platforms.m68k-linux"),
            NixPlatforms::M68kNetbsd => String::from("lib.platforms.m68k-netbsd"),
            NixPlatforms::M68kNone => String::from("lib.platforms.m68k-none"),
            NixPlatforms::MesaPlatforms => String::from("lib.platforms.mesaPlatforms"),
            NixPlatforms::Mips => String::from("lib.platforms.mips"),
            NixPlatforms::MipselLinux => String::from("lib.platforms.mipsel-linux"),
            NixPlatforms::MipselNetbsd => String::from("lib.platforms.mipsel-netbsd"),
            NixPlatforms::Mmix => String::from("lib.platforms.mmix"),
            NixPlatforms::MmixMmixware => String::from("lib.platforms.mmix-mmixware"),
            NixPlatforms::Msp430None => String::from("lib.platforms.msp430-none"),
            NixPlatforms::Netbsd => String::from("lib.platforms.netbsd"),
            NixPlatforms::None => String::from("lib.platforms.none"),
            NixPlatforms::Openbsd => String::from("lib.platforms.openbsd"),
            NixPlatforms::Or1k => String::from("lib.platforms.or1k"),
            NixPlatforms::Or1kNone => String::from("lib.platforms.or1k-none"),
            NixPlatforms::Powerpc64leLinux => String::from("lib.platforms.powerpc64le-linux"),
            NixPlatforms::Powerpc64Linux => String::from("lib.platforms.powerpc64-linux"),
            NixPlatforms::PowerpcNetbsd => String::from("lib.platforms.powerpc-netbsd"),
            NixPlatforms::PowerpcNone => String::from("lib.platforms.powerpc-none"),
            NixPlatforms::Redox => String::from("lib.platforms.redox"),
            NixPlatforms::Riscv => String::from("lib.platforms.riscv"),
            NixPlatforms::Riscv32Linux => String::from("lib.platforms.riscv32-linux"),
            NixPlatforms::Riscv32Netbsd => String::from("lib.platforms.riscv32-netbsd"),
            NixPlatforms::Riscv32None => String::from("lib.platforms.riscv32-none"),
            NixPlatforms::Riscv64Linux => String::from("lib.platforms.riscv64-linux"),
            NixPlatforms::Riscv64Netbsd => String::from("lib.platforms.riscv64-netbsd"),
            NixPlatforms::Riscv64None => String::from("lib.platforms.riscv64-none"),
            NixPlatforms::S390 => String::from("lib.platforms.s390"),
            NixPlatforms::S390Linux => String::from("lib.platforms.s390-linux"),
            NixPlatforms::S390None => String::from("lib.platforms.s390-none"),
            NixPlatforms::Unix => String::from("lib.platforms.unix"),
            NixPlatforms::Vc4 => String::from("lib.platforms.vc4"),
            NixPlatforms::Vc4None => String::from("lib.platforms.vc4-none"),
            NixPlatforms::Wasi => String::from("lib.platforms.wasi"),
            NixPlatforms::Wasm32Wasi => String::from("lib.platforms.wasm32-wasi"),
            NixPlatforms::Wasm64Wasi => String::from("lib.platforms.wasm64-wasi"),
            NixPlatforms::Windows => String::from("lib.platforms.windows"),
            NixPlatforms::X86 => String::from("lib.platforms.x86"),
            NixPlatforms::X86_64 => String::from("lib.platforms.x86-"),
            NixPlatforms::X86_64Cygwin => String::from("lib.platforms.x86-cygwin"),
            NixPlatforms::X86_64Darwin => String::from("lib.platforms.x86-darwin"),
            NixPlatforms::X86_64Freebsd => String::from("lib.platforms.x86-freebsd"),
            NixPlatforms::X86_64Genode => String::from("lib.platforms.x86-genode"),
            NixPlatforms::X86_64Linux => String::from("lib.platforms.x86-linux"),
            NixPlatforms::X86_64Netbsd => String::from("lib.platforms.x86-netbsd"),
            NixPlatforms::X86_64None => String::from("lib.platforms.x86-none"),
            NixPlatforms::X86_64Openbsd => String::from("lib.platforms.x86-openbsd"),
            NixPlatforms::X86_64Redox => String::from("lib.platforms.x86-redox"),
            NixPlatforms::X86_64Solaris => String::from("lib.platforms.x86-solaris"),
            NixPlatforms::X86_64Windows => String::from("lib.platforms.x86-windows"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_basic_conf() {
        let toml_str = r#"
        ["roscop.activefileinstatusbar"]
        src = "vsmarketplace"

        ["cometeer.spacemacs"]
        src = "vsmarketplace"

         "#
        .to_string();

        let actual = Package::get_packages(toml_str);

        let expected = vec![
            Package {
                name: "cometeer.spacemacs".to_string(),
                src: Sources::VSCodeMarketPlace,
            },
            Package {
                name: "roscop.activefileinstatusbar".to_string(),
                src: Sources::VSCodeMarketPlace,
            },
        ];
        assert_eq!(expected, actual)
    }
}
