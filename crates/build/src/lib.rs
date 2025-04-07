// Copyright (C) 2025 Ethan Uppal.
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use snafu::{whatever, ResultExt, Whatever};

/// Yes, I'm aware we are linking with the entire Vitalium library.
pub fn build_and_link_with_vitalium(submodule_path: &Path) -> Result<(), Whatever> {
    println!(
        "cargo:rerun-if-changed={}",
        submodule_path.join("meson.build").display()
    );

    let output_directory =
        PathBuf::from(env::var("OUT_DIR").whatever_context("Could not read OUT_DIR")?);
    let meson_build_directory = output_directory.join("meson_build");
    println!("cargo:rerun-if-changed={}", meson_build_directory.display());

    if !meson_build_directory.exists() {
        let status = Command::new("meson")
            .arg("setup")
            .arg(meson_build_directory.to_str().unwrap())
            .arg(submodule_path.to_str().unwrap())
            .status()
            .whatever_context("Failed to run `meson setup`")?;
        if !status.success() {
            whatever!("Failed to setup Meson build directory");
        }
    }

    let status = Command::new("meson")
        .arg("compile")
        .arg("-C")
        .arg(meson_build_directory.to_str().unwrap())
        .arg("vitalium")
        .status()
        .whatever_context("Failed to run `meson compile`")?;
    if !status.success() {
        whatever!("Failed to build Vitalium")
    }

    println!(
        "cargo:rustc-link-arg={}",
        meson_build_directory
            .join("ports-juce6.0/vitalium_lib.a")
            .display()
    );

    Ok(())
}
