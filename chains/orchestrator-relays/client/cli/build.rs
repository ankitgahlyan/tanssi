// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

fn main() {
    if let Ok(profile) = std::env::var("PROFILE") {
        println!("cargo:rustc-cfg=build_type=\"{}\"", profile);
    }
    substrate_build_script_utils::generate_cargo_keys();
    // For the node/worker version check, make sure we always rebuild the node when the version
    // changes.
    substrate_build_script_utils::rerun_if_git_head_changed();
}
