use syn::NestedMetaItem;
use syn::Lit;

use quote::ToTokens;
use quote::Tokens;

// Actived from
//   MetaItem::List("tag", Vec<NestedMetaItem>)
//   MetaItem::NameValue("tag", Lit)

pub enum Tags<'a> {
    One(Lit),
    Many(&'a [NestedMetaItem]),
}

fn lit_validate(lit: &Lit, tokens: &mut Tokens) {
    match *lit {
        Str(ref s, _) => {
            tokens.append(&format!(
                "if buffer.len() < {needed} { \
                 return Err(Error::InsufficientBytes) \
                 } \
                 let (_tmp, buffer) = buffer.split_at({needed}); \
                 if _tmp != {bytes} { \
                 return Err(Error::InvalidTag) \
                 }",
                needed = s.len(),
                bytes = s.as_bytes()
            )),
        },

        ByteStr(ref b, _) => {
            tokens.append(&format!(
                "if buffer.len() < {needed} { \
                 return Err(Error::InsufficientBytes) \
                 } \
                 let (_tmp, buffer) = buffer.split_at({needed}); \
                 if _tmp != {bytes} { \
                 return Err(Error::InvalidTag) \
                 }",
                needed = s.len(),
                bytes = s
            )),
        },

        Byte(b) => {
            tokens.append(&format!(
                
            ))
        }
    }
}

impl<'a> ToTokens for Tags<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {}
}
