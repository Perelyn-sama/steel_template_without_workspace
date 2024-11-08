pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;

pub mod prelude {
    pub use super::consts::*;
    pub use super::error::*;
    pub use super::instruction::*;
    pub use super::sdk::*;
    pub use super::state::*;

    pub use super::ID;
}

use steel::*;

// TODO Set program id
declare_id!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
