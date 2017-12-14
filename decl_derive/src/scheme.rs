#[derive(Declarative)]
#[declarative(arguments = "buffer: &'buf [u8]")]
pub struct Thing<'buf> {
    // Tags validate a match.  If there isn't a match,
    // the parser will return with an Error::InvalidTag
    #[tag("utf8 string")]   // tags can be strings
    #[tag(0x0001_0000u32)]  // tags can be a suffixed integer
    #[tag(b"cmap")]         // tags can be a byte str
    #[tag('k')]             // tags can be a character

    // Tags can match one of a number of options.
    #[tag(0x0001_0000u32, b"OTTO")]
    
    // Dropped fields will be parsed, and can be referenced
    // later on (by an Array for instance)
    #[dropped(num_tables = "u16")]

    // Arrays must be have a declared length, which either can
    // be an ident, or a constant integer.
    #[length = "num_tables")]
    #[length = 256]
    must_be_array: Array<'buf, T>,
    
    // Offsets can be relative to:
    // 1. The beginning of the table (default, no attributes)
    // 2. A reference, provided as an argument.
    #[relative_to(other_table = "&'buf [u8]")]
    
    // 3. The current position (which is named buffer)
    #[relative_to(buffer = "&'buf [u8]")]
    other_table: Offset32<'buf, OtherTable>,
    
    // Some tables may require an argument to be parsed
    #[argument(thing = "u32", buffer = "&'buf [u8]")]
    foo: Bar<'buf>,
}
