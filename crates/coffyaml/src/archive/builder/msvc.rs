use std::collections::HashMap;

use super::{
    ArchiveMemberIndex, ArchiveVariant, ByteSize, MemberSize,
    armap::ArchiveMapBuilder,
    longnames::{ArchiveLongNamesBuilder, ArchiveMemberName},
    sorted_armap::SortedArchiveMapBuilder,
};

#[derive(Default)]
pub struct MsvcArchiveVariant {
    /// The archive symbol map
    armap: ArchiveMapBuilder,

    sorted_armap: SortedArchiveMapBuilder,

    /// The long names
    longnames: ArchiveLongNamesBuilder<b'\0'>,
}

impl ByteSize for MsvcArchiveVariant {
    fn byte_size(&self) -> usize {
        let mut build_size = 0;

        // Archive map
        build_size += self.armap.member_size();

        // Sorted archive map
        build_size += self.sorted_armap.member_size();

        // Long names if it is not empty
        build_size += self.longnames.member_size();

        build_size
    }
}

impl ArchiveVariant for MsvcArchiveVariant {
    fn add_exported_symbol(&mut self, member: ArchiveMemberIndex, symbol: impl AsRef<str>) {
        self.armap.add_symbol(member, &symbol);
        self.sorted_armap.add_symbol(member, symbol);
    }

    fn add_long_name(&mut self, name: impl Into<String>) -> ArchiveMemberName {
        self.longnames.add_name(name)
    }

    fn build(self, archive_map: HashMap<ArchiveMemberIndex, usize>) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.byte_size());

        // Each member `.build()` method should add padding
        buffer.append(&mut self.armap.build(&archive_map));
        buffer.append(&mut self.sorted_armap.build(&archive_map));
        buffer.append(&mut self.longnames.build());

        buffer
    }
}
