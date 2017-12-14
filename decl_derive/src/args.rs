use quote::ToTokens;
use quote::Tokens;

use syn::Lit;
use syn::NestedMetaItem;
use syn::MetaItem;

// Maintain a collection of arguments. Keep track if this is for an Array
// which requires special formatting.
#[derive(Default)]
struct Arguments<'a> {
    is_array: Option<ArrayLength<'a>>,
    args: Vec<Argument<'a>>,
}

impl<'a> Arguments<'a> {
    fn type_value_tokens<'s>(&'s self) -> (ArgumentsType<'s>, ArgumentsValue<'s>) {
        (ArgumentsType(self), ArgumentsValue(self))
    }

    fn to_tokens_value(&self, tokens: &mut Tokens) {
        tokens.append("(");
        if let Some(ref length) = self.is_array {
            length.to_tokens_value(tokens);
            tokens.append(",(");
        }
        for arg in &self.args {
            arg.to_tokens_value(tokens);
            tokens.append(",");
        }
        if self.is_array.is_some() {
            tokens.append(")");
        }
        tokens.append(")");
    }
    
    fn to_tokens_type(&self, tokens: &mut Tokens) {
        tokens.append("(");
        if self.is_array.is_some() {
            tokens.append("usize,(");
        }
        for arg in &self.args {
            arg.to_tokens_type(tokens);
            tokens.append(",");
        }
        if self.is_array.is_some() {
            tokens.append(")");
        }
        tokens.append(")");
    }
    
    fn parse_arguments(items: &[NestedMetaItem]) -> Arguments {
        let mut arguments = Arguments::default();
        for item in items {
            let item = match *item {
                NestedMetaItem::MetaItem(ref item) => item,
                NestedMetaItem::Literal(_) => 
                    panic!("arguments must be a list of `<ident> = \"<type>\"`"),
            };
            
            let arg = Argument::from_meta_item(item);
            arguments.args.push(arg);
        }
        arguments
    }
}

struct ArgumentsType<'a>(&'a Arguments<'a>);

impl<'a> ToTokens for ArgumentsType<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.0.to_tokens_type(tokens);
    }
}

struct ArgumentsValue<'a>(&'a Arguments<'a>);

impl<'a> ToTokens for ArgumentsValue<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        self.0.to_tokens_value(tokens);
    }
}

// Used to construct arguments for the struct/enum as a
//   #[declarative(arguments = "buffer: &'buf [u8]")]
// or for a field, like
//   #[argument("first: u32", "second: &'buf [u8]")]
//
// Arguments must be referencable, either by being passed
// as an argument, a previous field, or a #[dropped(...)] attribute.
struct Argument<'a> {
    ident: &'a str,
    ty: &'a str,
}

impl<'a> Argument<'a> {
    fn to_tokens_value(&self, tokens: &mut Tokens) {
        tokens.append(self.ident);
    }
    
    fn to_tokens_type(&self, tokens: &mut Tokens) {
        tokens.append(self.ty);
    }
    
    fn from_meta_item(item: &MetaItem) -> Argument {
        match *item {
            MetaItem::NameValue(ref ident, ref lit) => {
                let ty = match *lit {
                    Lit::Str(ref ty, _) => ty,
                    _ => panic!("type declaration in argument must be a string."),
                };

                Argument {
                    ident: ident.as_ref(),
                    ty: ty,
                }
            },
            _ => panic!("arguments must be a list of `<ident> = \"<type>\"`"),
        }
    }
}

// Array lengths may either referenced a variable that has been parsed,
// or have constant size.
enum ArrayLength<'a> {
    Variable(&'a str),
    Constant(usize),
}

impl<'a> ArrayLength<'a> {
    fn to_tokens_value(&self, tokens: &mut Tokens) {
        match *self {
            ArrayLength::Variable(ref name) => tokens.append(name),
            ArrayLength::Constant(size) => size.to_tokens(tokens),
        }
    }
    
    fn to_tokens_type(&self, tokens: &mut Tokens) {
        tokens.append("usize");
    }
    
    fn from_lit(lit: &Lit) -> ArrayLength {
        match *lit {
            Lit::Str(ref ident, _) => ArrayLength::Variable(ident),
            Lit::Int(size, _) => ArrayLength::Constant(size as usize),
            _ => panic!("array lengths must be either a string or an integer.  Got: `{:?}`", lit),
        }
    }
}

#[test]
fn arguments_printing() {
    let arguments = Arguments {
        is_array: None,
        args: vec![
            Argument {
                ident: "buffer",
                ty: "&'buf [u8]",
            },
            
            Argument {
                ident: "num_tables",
                ty: "usize",
            }
        ],
    };
    
    let (ty, value) = arguments.type_value_tokens();
    let tokens = quote!( #value : #ty );
    assert_eq!("( buffer , num_tables , ) : ( &'buf [u8] , usize , )", tokens.as_str());
}

#[test]
fn arguments_array_printing() {
    let arguments = Arguments {
        is_array: Some(ArrayLength::Variable("num_glyphs")),
        args: vec![
            Argument {
                ident: "buffer",
                ty: "&'buf [u8]",
            },
            
            Argument {
                ident: "num_tables",
                ty: "usize",
            }
        ],
    };
    
    let (ty, value) = arguments.type_value_tokens();
    let tokens = quote!( #value : #ty );
    assert_eq!(
        "( num_glyphs ,( buffer , num_tables , ) ) : ( usize,( &\'buf [u8] , usize , ) )",
        tokens.as_str());
}
