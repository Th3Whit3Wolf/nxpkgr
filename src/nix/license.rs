use serde::{Deserialize, Serialize};

/*
    Licenses taken from
    https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix
*/

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum NixLicense {
    /// The Abstyles License.
    Abstyles,
    /// The Academic Free License v2.0.
    AFL_2_0,
    /// The Academic Free License v2.1.
    AFL_2_1,
    /// The Academic Free License v3.0.
    AFL_3_0,
    /// The GNU Affero General Public License v3.0.
    AGPL_3_0,
    /// The GNU Affero General Public License v3.0 only.
    AGPL_3_0_Only,
    /// The GNU Affero General Public License v3.0 or later.
    AGPL_3_0_Plus,
    /// AMD License Agreement.
    AMD,
    /// The Apache License 2.0
    Apache_2_0,
    /// The Apple Public Source License 2.0.
    APSL_2_0,
    /// The Artistic License 1.0.
    Artistic_1_0,
    /// The Artistic License 2.0.
    Artistic_2_0,
    /// Amazon Software License.
    ASL,
    /// The Beerware License.
    Beerware,
    /// The Blue Oak Model License 1.0.0.
    BlueOak_1_0_0,
    /// The BSD Zero Clause License.
    BSD_0,
    /// The BSD 1-Clause License.
    BSD_1,
    /// The BSD 2-Clause “Simplified” License.
    BSD_2,
    /// The BSD-2-Clause Plus Patent License.
    BSD_2_Patent,
    /// The BSD 3-Clause “New” or “Revised” License.
    BSD_3,
    /// The BSD 4-Clause “Original” or “Old” License.
    BSD_4,
    /// The BSD-4-Clause (University of California-Specific).
    BSD_4_UC,
    /// The BSD Protection License.
    BSD_Protection,
    /// The Boost Software License 1.0.
    BSL_1_0,
    /// The Business Source License 1.1.
    BUSL_1_1,
    /// The Clarified Artistic License.
    ClArtistic,
    /// The Creative Commons Zero v1.0 Universal.
    CC0_1_0,
    /// The Creative Commons Attribution Non Commercial Share Alike 2.0 Generic.
    CC_BY_NC_SA_2_0,
    /// The Creative Commons Attribution Non Commercial Share Alike 2.5 Generic.
    CC_BY_NC_SA_2_5,
    /// The Creative Commons Attribution Non Commercial Share Alike 3.0 Unported.
    CC_BY_NC_SA_3_0,
    /// The Creative Commons Attribution Non Commercial Share Alike 4.0 International.
    CC_BY_NC_SA_4_0,
    /// The Creative Commons Attribution Non Commercial 3.0 Unported.
    CC_BY_NC_3_0,
    /// The Creative Commons Attribution Non Commercial 4.0 International.
    CC_BY_NC_4_0,
    /// The Creative Commons Attribution No Derivatives 3.0 Unported.
    CC_BY_ND_3_0,
    /// The Creative Commons Attribution Share Alike 1.0 Generic.
    CC_BY_SA_2_5,
    /// The Creative Commons Attribution 3.0 Unported.
    CC_BY_3_0,
    /// The Creative Commons Attribution Share Alike 3.0 Unported.
    CC_BY_SA_3_0,
    /// The Creative Commons Attribution 4.0 International.
    CC_BY_4_0,
    /// The Creative Commons Attribution Share Alike 4.0 International.
    CC_BY_SA_4_0,
    /// The Common Development and Distribution License 1.0.
    CDDL_1_0,
    /// The CeCILL Free Software License Agreement v2.0.
    CECILL_2_0,
    /// The CeCILL-B Free Software License Agreement.
    CECILL_B,
    /// The CeCILL-C Free Software License Agreement.
    CECILL_C,
    /// The Common Public Attribution License 1.0.
    CPAL_1_0,
    /// The Common Public License 1.0.
    CPL_1_0,
    /// The curl License.
    Curl,
    /// Databricks Proprietary License
    DataBricks,
    /// The DOC License.
    DOC,
    /// "EPSON AVASYS PUBLIC LICENSE"
    EAPL,
    /// The Eiffel Forum License v1.0.
    EFL_1_0,
    /// The Eiffel Forum License v2.0.
    EFL_2_0,
    /// ELASTIC License
    Elastic,
    EPL_1_0,
    /// The Eclipse Public License 1.0.
    EPL_2_0,
    /// Seiko Epson Corporation Software License Agreement for Linux
    Epson,
    /// The European Union Public License 1.1.
    EUPL_1_1,
    /// The European Union Public License 1.2.
    EUPL_1_2,
    /// Floodgap Free Software License
    FFSL,
    /// Unspecified free software license
    Free,
    /// The Freetype Project License.
    FTL,
    /// Geant4 Software License
    G4SL,
    /// GeoGebra Non-Commercial License Agreement
    GeoGebra,
    /// The GNU Free Documentation License v1.1.
    GFDL_1_1,
    /// The GNU Free Documentation License v1.1 only - invariants.
    GFDL_1_1_Only,
    /// The GNU Free Documentation License v1.1 or later - invariants.
    GFDL_1_1_Plus,
    /// The GNU Free Documentation License v1.2.
    GFDL_1_2,
    /// The GNU Free Documentation License v1.2 only - invariants.
    GFDL_1_2_Only,
    /// The GNU Free Documentation License v1.2 or later - invariants.
    GFDL_1_2_Plus,
    /// The GNU Free Documentation License v1.3.
    GFDL_1_3,
    /// The GNU Free Documentation License v1.3 only - invariants.
    GFDL_1_3_Only,
    /// The GNU Free Documentation License v1.3 or later - invariants.
    GFDL_1_3_Plus,
    /// The GNU General Public License v1.0 only.
    GPL_1_0,
    /// The GNU General Public License v1.0 only.
    GPL_1_0_Only,
    /// The GNU General Public License v1.0 Plus.
    GPL_1_0_Plus,
    /// The GNU General Public License v2.0 only.
    GPL_2_0,
    /// The GNU General Public License v2.0 or later.
    GPL_2_0_Only,
    /// The GNU General Public License v2.0 w/Classpath exception.
    GPL_2_0_Classpath,
    /// The GNU General Public License v2.0 w/Linking exception.
    GPL_2_0_Linking,
    /// The GNU General Public License v2.0 or later.
    GPL_2_0_Plus,
    /// The GNU General Public License v2.0 or later w/Classpath exception.
    GPL_2_0_Plus_Classpath,
    /// The GNU General Public License v3.0 only.
    GPL_3_0,
    /// The GNU General Public License v3.0 or later.
    GPL_3_0_Only,
    /// The GNU General Public License v3.0 or later.
    GPL_3_0_Plus,
    /// The GNU General Public License v3.0 or later w/Classpath exception.
    GPL_3_0_Plus_Classpath,
    /// The Historical Permission Notice and Disclaimer.
    HPND,
    /// The Historical Permission Notice and Disclaimer - sell variant.
    HPND_Sell_Variant,
    /// The Intel ACPI Software License Agreement.
    Intel_ACPI,
    /// The Independent JPEG Group License.
    IJG,
    /// The ImageMagick License.
    ImageMagick,
    /// INRIA Non-Commercial License Agreement for the CompCert verified compiler
    InriaCompCert,
    /// INRIA Non-Commercial License Agreement for IceSL
    InriaIceSL,
    /// The Intel Open Source License.
    Intel,
    /// The IPA Font License.
    IPA,
    /// The IBM Public License v1.0.
    IPL_1_0,
    /// The ISC License.
    ISC,
    /// The GNU Library General Public License v2 only.
    LGPL_2_0,
    /// The GNU Library General Public License v2 only.
    LGPL_2_0_Only,
    /// The GNU Library General Public License v2 or later.
    LGPL_2_0_Plus,
    /// The GNU Library General Public License v2.1 only.
    LGPL_2_1,
    /// The GNU Library General Public License v2.1 only.
    LGPL_2_1_Only,
    /// The GNU Library General Public License v2.1 or later.
    LGPL_2_1_Plus,
    /// The GNU Library General Public License v3 only.
    LGPL_3_0,
    /// The GNU Library General Public License v3 only.
    LGPL_3_0_Only,
    /// The GNU Library General Public License v3 or later.
    LGPL_3_0_Plus,
    /// The Lesser General Public License For Linguistic Resources.
    LGPLLR,
    /// The libpng License.
    Libpng,
    /// The PNG Reference Library version 2.
    Libpng_2_0,
    /// The libtiff License.
    LibTiff,
    /// The LLVM Exception.
    LLVM_exception,
    /// Lisp LGPL; GNU Lesser General Public License version 2.1 with Franz Inc. preamble for clarification of LGPL terms in context of Lisp
    LLGPL_2_1,
    /// The LaTeX Project Public License v1.2.
    LPPL_1_2,
    /// The LaTeX Project Public License v1.3c.
    LPPL_1_3C,
    /// The Lucent Public License v1.02.
    LPL_1_02,
    /// The MirOS License.
    Miros,
    /// The MIT License.
    MIT,
    /// The Mozilla Public License 1.0.
    MPL_1_0,
    /// The Mozilla Public License 1.1.
    MPL_1_1,
    /// The Mozilla Public License 2.0.
    MPL_2_0,
    /// The Microsoft Public License.
    MS_PL,
    /// The NASA Open Source Agreement 1.3.
    NASA_1_3,
    /// The University of Illinois/NCSA Open Source License.
    NCSA,
    /// The Non-Profit Open Software License 3.0.
    NPOSL_3_0,
    /// Obsidian End User Agreement
    Obsidian,
    /// OCamlPro Non Commercial license version 1
    OCamlPro_NC,
    /// The Open Data Commons Open Database License v1.0.
    ODbL_1_0,
    /// The SIL Open Font License 1.0.
    OFL_1_0,
    /// The Open LDAP Public License v2.8.
    OLDAP_2_8,
    /// The OpenSSL License.
    OpenSSL,
    /// The Open Software License 2.0.
    OSL_2_0,
    /// The Open Software License 2.1.
    OSL_2_1,
    /// The Open Software License 3.0.
    OSL_3_0,
    /// The The Parity Public License 7.0.0.
    Parity_7_0_0,
    /// The PHP License v3.01.
    PHP_3_01,
    /// The PostgreSQL License.
    PostgreSQL,
    /// Postman EULA
    Postman,
    /// The Python Software Foundation License 2.0.
    PSF_2_0,
    /// Public Domain
    PublicDomain,
    /// Purdue BSD-Style License
    PurdueBsd,
    /// Prosperity-3.0.0 License.
    Prosperity_3_0_0,
    /// The Qhull License.
    Qhull,
    /// The Q Public License 1.0.
    QPL_1_0,
    /// The Qwt License v1.0
    Qwt,
    /// The Ruby license.
    Ruby,
    /// The Sendmail License.
    Sendmail,
    /// The SGI Free Software License B v2.0.
    SGI_B_2_0,
    /// The Sleepycat License.
    Sleepycat,
    /// SMAIL General Public License
    Smail,
    /// The Server Side Public License, v 1.
    SSPL_1_0,
    /// The Synthesis Tool Kit 4.3
    STK_4_3,
    /// The TCL/TK License.
    TCL,
    /// The Ubuntu Font License 1.0
    UFL_1_0,
    /// Unfree
    UnFree,
    /// Unfree redistributable
    UnFreeRedistributable,
    /// Unfree redistributable firmware
    UnFreeRedistributableFirmware,
    /// The Unicode License Agreement - Data Files and Software (2015).
    Unicode_DFS_2015,
    /// The Unicode License Agreement - Data Files and Software (2015).
    Unicode_DFS_2016,
    /// The The Unlicense.
    Unlicense,
    /// The Universal Permissive License v1.0
    UPL_1_0,
    /// The Vim License.
    Vim,
    /// Oracle VM VirtualBox Extension Pack Personal Use and Evaluation License (PUEL)
    VirtualBox_PUEL,
    /// The Vovida Software License v1.0.
    VSL_1_0,
    /// The Sybase Open Watcom Public License 1.0.
    Watcom_1_0,
    /// The W3C Software Notice and License (2002-12-31).
    W3C,
    /// The Do What The F*ck You Want To Public License.
    WTFPL,
    /// The WxWindows Library Exception 3.1.
    WxWindows_Exception_3_1,
    /// The zlib License.
    Zlib,
    /// The Zope Public License 2.0.
    ZPL_2_0,
    /// The Zope Public License 2.1.
    ZPL_2_1,
}

impl NixLicense {
    pub fn from_str(s: &str) -> Option<&'static Self> {
        match s {
            "Abstyles" => Some(&NixLicense::Abstyles),
            "AFL-2.0" => Some(&NixLicense::AFL_2_0),
            "AFL-2.1" => Some(&NixLicense::AFL_2_1),
            "AFL-3.0" => Some(&NixLicense::AFL_3_0),
            "AGPL-3.0" => Some(&NixLicense::AGPL_3_0),
            "AGPL-3.0-only" => Some(&NixLicense::AGPL_3_0_Only),
            "AGPL-3.0-or-later" => Some(&NixLicense::AGPL_3_0_Plus),
            "Apache-2.0" => Some(&NixLicense::Apache_2_0),
            "APSL-2.0" => Some(&NixLicense::APSL_2_0),
            "Artistic-1.0" => Some(&NixLicense::Artistic_1_0),
            "Artistic-2.0" => Some(&NixLicense::Artistic_2_0),
            "Beerware" => Some(&NixLicense::Beerware),
            "BlueOak-1.0.0" => Some(&NixLicense::BlueOak_1_0_0),
            "0BSD" => Some(&NixLicense::BSD_0),
            "BSD-1-Clause" => Some(&NixLicense::BSD_1),
            "BSD-2-Clause" => Some(&NixLicense::BSD_2),
            "BSD-2-Clause-Patent" => Some(&NixLicense::BSD_2_Patent),
            "BSD-3-Clause" => Some(&NixLicense::BSD_3),
            "BSD-4-Clause" => Some(&NixLicense::BSD_4),
            "BSD-4-Clause-UC" => Some(&NixLicense::BSD_4_UC),
            "BSD-Protection" => Some(&NixLicense::BSD_Protection),
            "BSL-1.0" => Some(&NixLicense::BSL_1_0),
            "BUSL-1.1" => Some(&NixLicense::BUSL_1_1),
            "ClArtistic" => Some(&NixLicense::ClArtistic),
            "CC0-1.0" => Some(&NixLicense::CC0_1_0),
            "CC-BY-NC-SA-2.0" => Some(&NixLicense::CC_BY_NC_SA_2_0),
            "CC-BY-NC-SA-2.5" => Some(&NixLicense::CC_BY_NC_SA_2_5),
            "CC-BY-NC-SA-3.0" => Some(&NixLicense::CC_BY_NC_SA_3_0),
            "CC-BY-NC-SA-4.0" => Some(&NixLicense::CC_BY_NC_SA_4_0),
            "CC-BY-NC-3.0" => Some(&NixLicense::CC_BY_NC_3_0),
            "CC-BY-NC-4.0" => Some(&NixLicense::CC_BY_NC_4_0),
            "CC-BY-ND-3.0" => Some(&NixLicense::CC_BY_ND_3_0),
            "CC-BY-SA-2.5" => Some(&NixLicense::CC_BY_SA_2_5),
            "CC-BY-3.0-AT" => Some(&NixLicense::CC_BY_3_0),
            "CC-BY-SA-3.0-AT" => Some(&NixLicense::CC_BY_SA_3_0),
            "CC-BY-4.0" => Some(&NixLicense::CC_BY_4_0),
            "CC-BY-SA-4.0" => Some(&NixLicense::CC_BY_SA_4_0),
            "CDDL-1.0" => Some(&NixLicense::CDDL_1_0),
            "CECILL-2.0" => Some(&NixLicense::CECILL_2_0),
            "CECILL-B" => Some(&NixLicense::CECILL_B),
            "CECILL-C" => Some(&NixLicense::CECILL_C),
            "CPAL-1.0" => Some(&NixLicense::CPAL_1_0),
            "CPL-1.0" => Some(&NixLicense::CPL_1_0),
            "curl" => Some(&NixLicense::Curl),
            "DOC" => Some(&NixLicense::DOC),
            "EFL-1.0" => Some(&NixLicense::EFL_1_0),
            "EFL-2.0" => Some(&NixLicense::EFL_2_0),
            "EPL-1.0" => Some(&NixLicense::EPL_1_0),
            "EPL-2.0" => Some(&NixLicense::EPL_2_0),
            "EUPL-1.1" => Some(&NixLicense::EUPL_1_1),
            "EUPL-1.2" => Some(&NixLicense::EUPL_1_2),
            "FTL" => Some(&NixLicense::FTL),
            "GFDL-1.1" => Some(&NixLicense::GFDL_1_1),
            "GFDL-1.1-only" => Some(&NixLicense::GFDL_1_1_Only),
            "GFDL-1.1-or-later" => Some(&NixLicense::GFDL_1_1_Plus),
            "GFDL-1.2" => Some(&NixLicense::GFDL_1_2),
            "GFDL-1.2-only" => Some(&NixLicense::GFDL_1_2_Only),
            "GFDL-1.2-or-later" => Some(&NixLicense::GFDL_1_2_Plus),
            "GFDL-1.3" => Some(&NixLicense::GFDL_1_3),
            "GFDL-1.3-only" => Some(&NixLicense::GFDL_1_3_Only),
            "GFDL-1.3-or-later" => Some(&NixLicense::GFDL_1_3_Plus),
            "GPL-1.0" => Some(&NixLicense::GPL_1_0),
            "GPL-1.0-only" => Some(&NixLicense::GPL_1_0_Only),
            "GPL-1.0+" => Some(&NixLicense::GPL_1_0_Plus),
            "GPL-2.0" => Some(&NixLicense::GPL_2_0),
            "GPL-2.0-only" => Some(&NixLicense::GPL_2_0_Only),
            "GPL-2.0-with-classpath-exception" => Some(&NixLicense::GPL_2_0_Classpath),
            "GPL-2.0-or-later" => Some(&NixLicense::GPL_2_0_Plus),
            "GPL-3.0" => Some(&NixLicense::GPL_3_0),
            "GPL-3.0-only" => Some(&NixLicense::GPL_3_0_Only),
            "GPL-3.0+" | "GPL-3.0-or-later" => Some(&NixLicense::GPL_3_0_Plus),
            "GPL-3.0-linking-exception" => Some(&NixLicense::GPL_3_0_Plus_Classpath),
            "HPND" => Some(&NixLicense::HPND),
            "HPND-sell-variant" => Some(&NixLicense::HPND_Sell_Variant),
            "Intel-ACPI" => Some(&NixLicense::Intel_ACPI),
            "IJG" => Some(&NixLicense::IJG),
            "ImageMagick" => Some(&NixLicense::ImageMagick),
            "Intel" => Some(&NixLicense::Intel),
            "IPA" => Some(&NixLicense::IPA),
            "IPL-1.0" => Some(&NixLicense::IPL_1_0),
            "ISC" => Some(&NixLicense::ISC),
            "LGPL-2.0" => Some(&NixLicense::LGPL_2_0),
            "LGPL-2.0-only" => Some(&NixLicense::LGPL_2_0_Only),
            "LGPL-2.0-or-later" => Some(&NixLicense::LGPL_2_0_Plus),
            "LGPL-2.1" => Some(&NixLicense::LGPL_2_1),
            "LGPL-2.1-only" => Some(&NixLicense::LGPL_2_1_Only),
            "LGPL-2.1+" | "LGPL-2.1-or-later" => Some(&NixLicense::LGPL_2_1_Plus),
            "LGPL-3.0" => Some(&NixLicense::LGPL_3_0),
            "LGPL-3.0-only" => Some(&NixLicense::LGPL_3_0_Only),
            "LGPL-3.0-or-later" | "LGPL-3.0+" => Some(&NixLicense::LGPL_3_0_Plus),
            "LGPLLR" => Some(&NixLicense::LGPLLR),
            "Libpng" => Some(&NixLicense::Libpng),
            "libpng-2.0" => Some(&NixLicense::Libpng_2_0),
            "libtiff" => Some(&NixLicense::LibTiff),
            "LLVM-exception" => Some(&NixLicense::LLVM_exception),
            "LPPL-1.2" => Some(&NixLicense::LPPL_1_2),
            "LPPL-1.3a" => Some(&NixLicense::LPPL_1_3C),
            "LPL-1.02" => Some(&NixLicense::LPL_1_02),
            "MirOS" => Some(&NixLicense::Miros),
            "MIT" => Some(&NixLicense::MIT),
            "MPL-1.0" => Some(&NixLicense::MPL_1_0),
            "MPL-1.1" => Some(&NixLicense::MPL_1_1),
            "MPL-2.0" => Some(&NixLicense::MPL_2_0),
            "MS-PL" => Some(&NixLicense::MS_PL),
            "NASA-1.3" => Some(&NixLicense::NASA_1_3),
            "NCSA" => Some(&NixLicense::NCSA),
            "NPOSL-3.0" => Some(&NixLicense::NPOSL_3_0),
            "ODbL-1.0" => Some(&NixLicense::ODbL_1_0),
            "OFL-1.0" => Some(&NixLicense::OFL_1_0),
            "OLDAP-2.8" => Some(&NixLicense::OLDAP_2_8),
            "OpenSSL" => Some(&NixLicense::OpenSSL),
            "OSL-2.0" => Some(&NixLicense::OSL_2_0),
            "OSL-2.1" => Some(&NixLicense::OSL_2_1),
            "OSL-3.0" => Some(&NixLicense::OSL_3_0),
            "Parity-7.0.0" => Some(&NixLicense::Parity_7_0_0),
            "PHP-3.01" => Some(&NixLicense::PHP_3_01),
            "PostgreSQL" => Some(&NixLicense::PostgreSQL),
            "Python-2.0" => Some(&NixLicense::PSF_2_0),
            "Qhull" => Some(&NixLicense::Qhull),
            "QPL-1.0" => Some(&NixLicense::QPL_1_0),
            "Ruby" => Some(&NixLicense::Ruby),
            "Sendmail" => Some(&NixLicense::Sendmail),
            "SGI-B-2.0" => Some(&NixLicense::SGI_B_2_0),
            "Sleepycat" => Some(&NixLicense::Sleepycat),
            "SSPL-1.0" => Some(&NixLicense::SSPL_1_0),
            "TCL" => Some(&NixLicense::TCL),
            "Unicode-DFS-2015" => Some(&NixLicense::Unicode_DFS_2015),
            "Unicode-DFS-2016" => Some(&NixLicense::Unicode_DFS_2016),
            "Unlicense" => Some(&NixLicense::Unlicense),
            "UPL-1.0" => Some(&NixLicense::UPL_1_0),
            "Vim" => Some(&NixLicense::Vim),
            "VSL-1.0" => Some(&NixLicense::VSL_1_0),
            "Watcom-1.0" => Some(&NixLicense::Watcom_1_0),
            "W3C" => Some(&NixLicense::W3C),
            "WTFPL" => Some(&NixLicense::WTFPL),
            "wxWindows" => Some(&NixLicense::WxWindows_Exception_3_1),
            "Zlib" => Some(&NixLicense::Zlib),
            "ZPL-2.0" => Some(&NixLicense::ZPL_2_0),
            "ZPL-2.1" => Some(&NixLicense::ZPL_2_1),
            _ => None,
        }
    }
    pub fn to_nix_meta(self) -> &'static str {
        match self {
            NixLicense::Abstyles => "lib.licenses.abstyles",
            NixLicense::AFL_2_0 => "lib.licenses.afl20",
            NixLicense::AFL_2_1 => "lib.licenses.afl21",
            NixLicense::AFL_3_0 => "lib.licenses.afl3",
            NixLicense::AGPL_3_0 => "lib.licenses.agpl3",
            NixLicense::AGPL_3_0_Only => "lib.licenses.agpl3Only",
            NixLicense::AGPL_3_0_Plus => "lib.licenses.agpl3Plus",
            NixLicense::AMD => "lib.licenses.amd",
            NixLicense::Apache_2_0 => "lib.licenses.asl20",
            NixLicense::APSL_2_0 => "lib.licenses.apsl20",
            NixLicense::Artistic_1_0 => "lib.licenses.artistic1",
            NixLicense::Artistic_2_0 => "lib.licenses.artistic2",
            NixLicense::ASL => "lib.licenses.amazonsl",
            NixLicense::Beerware => "lib.licenses.beerware",
            NixLicense::BlueOak_1_0_0 => "lib.licenses.blueOak100",
            NixLicense::BSD_0 => "lib.licenses.bsd0",
            NixLicense::BSD_1 => "lib.licenses.bsd1",
            NixLicense::BSD_2 => "lib.licenses.bsd2",
            NixLicense::BSD_2_Patent => "lib.licenses.bsd2Patent",
            NixLicense::BSD_3 => "lib.licenses.bsd3",
            NixLicense::BSD_4 => "lib.licenses.bsdOriginal",
            NixLicense::BSD_4_UC => "lib.licenses.bsdOriginalUC",
            NixLicense::BSD_Protection => "lib.licenses.bsdProtection",
            NixLicense::BSL_1_0 => "lib.licenses.boost",
            NixLicense::BUSL_1_1 => "lib.licenses.bsl11",
            NixLicense::ClArtistic => "lib.licenses.clArtistic",
            NixLicense::CC0_1_0 => "lib.licenses.cc0",
            NixLicense::CC_BY_NC_SA_2_0 => "lib.licenses.cc-by-nc-sa-20",
            NixLicense::CC_BY_NC_SA_2_5 => "lib.licenses.cc-by-nc-sa-25",
            NixLicense::CC_BY_NC_SA_3_0 => "lib.licenses.cc-by-nc-sa-30",
            NixLicense::CC_BY_NC_SA_4_0 => "lib.licenses.cc-by-nc-sa-40",
            NixLicense::CC_BY_NC_3_0 => "lib.licenses.cc-by-nc-30",
            NixLicense::CC_BY_NC_4_0 => "lib.licenses.cc-by-nc-40",
            NixLicense::CC_BY_ND_3_0 => "lib.licenses.cc-by-nd-30",
            NixLicense::CC_BY_SA_2_5 => "lib.licenses.cc-by-sa-25",
            NixLicense::CC_BY_3_0 => "lib.licenses.cc-by-30",
            NixLicense::CC_BY_SA_3_0 => "lib.licenses.cc-by-sa-30",
            NixLicense::CC_BY_4_0 => "lib.licenses.cc-by-40",
            NixLicense::CC_BY_SA_4_0 => "lib.licenses.cc-by-sa-40",
            NixLicense::CDDL_1_0 => "lib.licenses.cddl",
            NixLicense::CECILL_2_0 => "lib.licenses.cecill20",
            NixLicense::CECILL_B => "lib.licenses.cecill-b",
            NixLicense::CECILL_C => "lib.licenses.cecill-c",
            NixLicense::CPAL_1_0 => "lib.licenses.cpal10",
            NixLicense::CPL_1_0 => "lib.licenses.cpl10",
            NixLicense::Curl => "lib.licenses.curl",
            NixLicense::DataBricks => "lib.licenses.databricks",
            NixLicense::DOC => "lib.licenses.doc",
            NixLicense::EAPL => "lib.licenses.eapl",
            NixLicense::EFL_1_0 => "lib.licenses.efl10",
            NixLicense::EFL_2_0 => "lib.licenses.efl20",
            NixLicense::Elastic => "lib.licenses.elastic",
            NixLicense::EPL_1_0 => "lib.licenses.epl10",
            NixLicense::EPL_2_0 => "lib.licenses.epl20",
            NixLicense::Epson => "lib.licenses.epson",
            NixLicense::EUPL_1_1 => "lib.licenses.eupl11",
            NixLicense::EUPL_1_2 => "lib.licenses.eupl12",
            NixLicense::FFSL => "lib.licenses.ffsl",
            NixLicense::Free => "lib.licenses.free",
            NixLicense::FTL => "lib.licenses.ftl",
            NixLicense::G4SL => "lib.licenses.g4sl",
            NixLicense::GeoGebra => "lib.licenses.geogebra",
            NixLicense::GFDL_1_1 => "lib.licenses.fdl11",
            NixLicense::GFDL_1_1_Only => "lib.licenses.fdl11Only",
            NixLicense::GFDL_1_1_Plus => "lib.licenses.fdl11Plus",
            NixLicense::GFDL_1_2 => "lib.licenses.fdl12",
            NixLicense::GFDL_1_2_Only => "lib.licenses.fdl12Only",
            NixLicense::GFDL_1_2_Plus => "lib.licenses.fdl12Plus",
            NixLicense::GFDL_1_3 => "lib.licenses.fdl13",
            NixLicense::GFDL_1_3_Only => "lib.licenses.fdl13Only",
            NixLicense::GFDL_1_3_Plus => "lib.licenses.fdl13Plus",
            NixLicense::GPL_1_0 => "lib.licenses.gpl1",
            NixLicense::GPL_1_0_Only => "lib.licenses.gpl1Only",
            NixLicense::GPL_1_0_Plus => "lib.licenses.gpl1Plus",
            NixLicense::GPL_2_0 => "lib.licenses.gpl2",
            NixLicense::GPL_2_0_Only => "lib.licenses.gpl2Only",
            NixLicense::GPL_2_0_Classpath => "lib.licenses.gpl2Classpath",
            NixLicense::GPL_2_0_Plus => "lib.licenses.gpl2Plus",
            NixLicense::GPL_3_0 => "lib.licenses.gpl3",
            NixLicense::GPL_3_0_Only => "lib.licenses.gpl3Only",
            NixLicense::GPL_3_0_Plus => "lib.licenses.gpl3Plus",
            NixLicense::HPND => "lib.licenses.hpnd",
            NixLicense::HPND_Sell_Variant => "lib.licenses.hpndSellVariant",
            NixLicense::IJG => "lib.licenses.ijg",
            NixLicense::ImageMagick => "lib.licenses.imagemagick",
            NixLicense::IPA => "lib.licenses.ipa",
            NixLicense::IPL_1_0 => "lib.licenses.ipl10",
            NixLicense::ISC => "lib.licenses.isc",
            NixLicense::LGPL_2_0 => "lib.licenses.lgpl2",
            NixLicense::LGPL_2_0_Only => "lib.licenses.lgpl2Only",
            NixLicense::LGPL_2_0_Plus => "lib.licenses.lgpl2Plus",
            NixLicense::LGPL_2_1 => "lib.licenses.lgpl21",
            NixLicense::LGPL_2_1_Only => "lib.licenses.lgpl21Only",
            NixLicense::LGPL_2_1_Plus => "lib.licenses.lgpl21Plus",
            NixLicense::LGPL_3_0 => "lib.licenses.lgpl3",
            NixLicense::LGPL_3_0_Only => "lib.licenses.lgpl3Only",
            NixLicense::LGPL_3_0_Plus => "lib.licenses.lgpl3Plus",
            NixLicense::LGPLLR => "lib.licenses.lgpllr",
            NixLicense::Libpng => "lib.licenses.libpng",
            NixLicense::Libpng_2_0 => "lib.licenses.libpng2",
            NixLicense::LibTiff => "lib.licenses.libtiff",
            NixLicense::LLVM_exception => "lib.licenses.llvm-exception",
            NixLicense::LPPL_1_2 => "lib.licenses.lppl12",
            NixLicense::LPPL_1_3C => "lib.licenses.lppl13c",
            NixLicense::LPL_1_02 => "lib.licenses.lpl-102",
            NixLicense::MIT => "lib.licenses.mit",
            NixLicense::MPL_1_0 => "lib.licenses.mpl10",
            NixLicense::MPL_1_1 => "lib.licenses.mpl11",
            NixLicense::MPL_2_0 => "lib.licenses.mpl20",
            NixLicense::MS_PL => "lib.licenses.mspl",
            NixLicense::NASA_1_3 => "lib.licenses.nasa13",
            NixLicense::NCSA => "lib.licenses.ncsa",
            NixLicense::NPOSL_3_0 => "lib.licenses.nposl3",
            NixLicense::ODbL_1_0 => "lib.licenses.odbl",
            NixLicense::OFL_1_0 => "lib.licenses.ofl",
            NixLicense::OLDAP_2_8 => "lib.licenses.openldap",
            NixLicense::OpenSSL => "lib.licenses.openssl",
            NixLicense::OSL_2_0 => "lib.licenses.osl2",
            NixLicense::OSL_2_1 => "lib.licenses.osl21",
            NixLicense::OSL_3_0 => "lib.licenses.osl3",
            NixLicense::Parity_7_0_0 => "lib.licenses.parity70",
            NixLicense::PHP_3_01 => "lib.licenses.php301",
            NixLicense::PostgreSQL => "lib.licenses.postgresql",
            NixLicense::PSF_2_0 => "lib.licenses.psfl",
            NixLicense::Qhull => "lib.licenses.qhull",
            NixLicense::QPL_1_0 => "lib.licenses.qpl",
            NixLicense::Ruby => "lib.licenses.ruby",
            NixLicense::Sendmail => "lib.licenses.sendmail",
            NixLicense::SGI_B_2_0 => "lib.licenses.sgi-b-20",
            NixLicense::Sleepycat => "lib.licenses.",
            NixLicense::TCL => "lib.licenses.sleepycat",
            NixLicense::Unicode_DFS_2015 => "lib.licenses.unicode-dfs-2015",
            NixLicense::Unicode_DFS_2016 => "lib.licenses.unicode-dfs-2016",
            NixLicense::Unlicense => "lib.licenses.unlicense",
            NixLicense::Vim => "lib.licenses.vim",
            NixLicense::VSL_1_0 => "lib.licenses.vsl10",
            NixLicense::Watcom_1_0 => "lib.licenses.watcom",
            NixLicense::W3C => "lib.licenses.w3c",
            NixLicense::WTFPL => "lib.licenses.wtfpl",
            NixLicense::WxWindows_Exception_3_1 => "lib.licenses.wxWindows",
            NixLicense::Zlib => "lib.licenses.zlib",
            NixLicense::ZPL_2_0 => "lib.licenses.zpl20",
            NixLicense::ZPL_2_1 => "lib.licenses.zpl21",
            NixLicense::GPL_2_0_Linking => "lib.licenses.gpl2Oss",
            NixLicense::GPL_2_0_Plus_Classpath => "lib.licenses.gpl2ClasspathPlus",
            NixLicense::GPL_3_0_Plus_Classpath => "lib.licenses.gpl3ClasspathPlus",
            NixLicense::Intel_ACPI => "lib.licenses.iasl",
            NixLicense::InriaCompCert => "lib.licenses.inria-compcert",
            NixLicense::InriaIceSL => "lib.licenses.inria-icesl",
            NixLicense::Intel => "lib.licenses.issl",
            NixLicense::LLGPL_2_1 => "lib.licenses.llgpl21",
            NixLicense::Miros => "lib.licenses.miros",
            NixLicense::Obsidian => "lib.licenses.obsidian",
            NixLicense::OCamlPro_NC => "lib.licenses.ocamlpro_nc",
            NixLicense::Postman => "lib.licenses.postman",
            NixLicense::PublicDomain => "lib.licenses.publicDomain",
            NixLicense::PurdueBsd => "lib.licenses.purdueBsd",
            NixLicense::Prosperity_3_0_0 => "lib.licenses.prosperity30",
            NixLicense::Qwt => "lib.licenses.qwt",
            NixLicense::Smail => "lib.licenses.smail",
            NixLicense::SSPL_1_0 => "lib.licenses.sspl",
            NixLicense::STK_4_3 => "lib.licenses.stk",
            NixLicense::UFL_1_0 => "lib.licenses.ufl",
            NixLicense::UnFree => "lib.licenses.unfree",
            NixLicense::UnFreeRedistributable => "lib.licenses.unfreeRedistributable",
            NixLicense::UnFreeRedistributableFirmware => {
                "lib.licenses.unfreeRedistributableFirmware"
            }
            NixLicense::UPL_1_0 => "lib.licenses.upl",
            NixLicense::VirtualBox_PUEL => "lib.licenses.virtualbox-puel",
        }
    }
}
