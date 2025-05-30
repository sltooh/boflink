use std::collections::HashMap;

use super::{
    ArchiveMemberIndex, ArchiveVariant, ByteSize, MemberSize,
    armap::ArchiveMapBuilder,
    longnames::{ArchiveLongNamesBuilder, ArchiveMemberName},
};

#[derive(Default)]
pub struct GnuArchiveVariant {
    armap: ArchiveMapBuilder,
    longnames: ArchiveLongNamesBuilder<b'\n'>,
}

impl ByteSize for GnuArchiveVariant {
    fn byte_size(&self) -> usize {
        let mut build_size = 0;

        // Archive map
        build_size += self.armap.member_size();

        // Long names
        build_size += self.longnames.member_size();

        build_size
    }
}

impl ArchiveVariant for GnuArchiveVariant {
    fn add_exported_symbol(&mut self, member: ArchiveMemberIndex, symbol: impl AsRef<str>) {
        self.armap.add_symbol(member, symbol);
    }

    fn add_long_name(&mut self, name: impl Into<String>) -> ArchiveMemberName {
        self.longnames.add_name(name)
    }

    fn build(self, archive_map: HashMap<ArchiveMemberIndex, usize>) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(self.byte_size());
        buffer.append(&mut self.armap.build(&archive_map));
        buffer.append(&mut self.longnames.build());

        buffer
    }
}
