/// A reference to an existing value
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference(pub(crate) u16);

impl Reference {
    /// Returns the index of the referenced value in the AMF0 reference table.
    pub const fn index(self) -> u16 {
        self.0
    }
}
