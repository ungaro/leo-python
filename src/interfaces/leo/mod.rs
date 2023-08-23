// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// local program commands
//pub mod build;
//pub use build::Build;

//pub mod clean;
//pub use clean::Clean;

// pub mod deploy;
// pub use deploy::Deploy;

pub mod new;
pub use new::New;

// pub mod node;
// pub use node::Node;

pub mod run;
pub use run::Run;

use crate::context::*;
use leo_errors::Result;

use tracing::span::Span;

pub(crate) type Network = snarkvm::prelude::Testnet3;
pub(crate) const ALEO_CLI_COMMAND: &str = "aleo";