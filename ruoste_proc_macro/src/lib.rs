use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Virh" => "Err",
        "Okei" => "Ok",
        "Lanka" => "String",
        "RisuaitaKartta" => "HashMap",
        "Oletus" => "Default",
        "Virhe" => "Error",
        "Mahdollisuus" => "Option",
        "Jokin" => "Some",
        "EiMikään" => "None",
        "Tulos" => "Result",
        "Itse" => "Self",
        "tulosta" => "println",
        "keskeytä" => "break",
        "asynkroninen" => "async",
        "odota" => "await",
        "silmukka" => "loop",
        "liikuta" => "move",
        "kori" => "crate",
        "saavuttamaton_koodi" => "unreachable_code",
        "kuin" => "as",
        "vakio" => "const",
        "piirre" => "trait",
        "turvaton" => "unsafe",
        "sisälle" => "in",
        "jostakin" => "from",
        "dynaaminen" => "dyn",
        "kääri_esiin" => "unwrap",
        "oletus" => "default",
        "viittaukseksi" => "as_ref",
        "siirräntä" => "io",
        "ulkoinen" => "extern",
        "epätosi" => "false",
        "funktio" => "fn",
        "loistava" => "super",
        "sijoita" => "insert",
        "ota" => "get",
        "salli" => "allow",
        "voi_perkele" | "oho" | "hups" => "panic",
        "moduuli" => "mod",
        "muuttuva" => "mut",
        "uusi" => "new",
        "missä" => "where",
        "kaikille" => "for",
        "ota_tai_sijoita_käyttäen" => "get_or_insert_with",
        "alku" => "main",
        "julkinen" => "pub",
        "EiMitäänkö" => None?,
        "palauta" => "return",
        "toteutus" => "impl",
        "viittaus" => "ref",
        "täsmää" => "match",
        "jos" => "if",
        "muuten" => "else",
        "itse" => "self",
        "olkoon" => "let",
        "staattinen" => "static",
        "rakenne" => "struct",
        "edellytä" => "expect",
        "kunnes" => "while",
        "käytä" => "use",
        "osaksi" => "into",
        "tosi" => "true",
        "luettelo" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ruoste(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
