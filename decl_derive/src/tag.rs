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
        Str(ref s, _) => quote!( lit ),
        ByteStr(ref b, _) => quote! ( lit ),
        Byte(b) => {
            tokens.append(&format!(

            ))
        }
    }
}

impl<'a> ToTokens for Tags<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {}
}
