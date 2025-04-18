use packed_struct::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PackedStruct, Serialize, Deserialize)]
#[packed_struct(bit_numbering = "msb0")]
pub struct PackedMazeCell {
    #[packed_field(bits = "0:1")]
    version: Integer<u8, packed_bits::Bits<2>>,
    #[packed_field(bits = "4")]
    up_blocked: bool,
    #[packed_field(bits = "5")]
    right_blocked: bool,
    #[packed_field(bits = "6")]
    down_blocked: bool,
    #[packed_field(bits = "7")]
    left_blocked: bool,
}

#[cfg(test)]
mod tests {
    use super::PackedMazeCell;
    use packed_struct::prelude::*;

    #[test]
    fn test_packing() -> Result<(), String> {
        let unpacked = PackedMazeCell {
            version: 1.into(),
            up_blocked: true,
            right_blocked: false,
            down_blocked: false,
            left_blocked: false,
        };

        // pack into a byte array
        let packed = unpacked.pack().expect("Should have successfully packed");
        assert_eq!([0b01001000], packed);

        // unpack from a byte array
        let unpacked = PackedMazeCell::unpack(&packed).expect("Should have successfully unpacked");
        assert_eq!(*unpacked.version, 1);
        assert_eq!(unpacked.up_blocked, true);
        assert_eq!(unpacked.right_blocked, false);
        assert_eq!(unpacked.down_blocked, false);
        assert_eq!(unpacked.left_blocked, false);

        Ok(())
    }
}
