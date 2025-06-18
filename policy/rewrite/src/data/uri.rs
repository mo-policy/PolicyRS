// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

// https://www.rfc-editor.org/rfc/rfc3986

use std::{fmt::Display, str::{Chars, FromStr}};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DidUri {
    pub text: String,
    pub parts: Vec<DidUriPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DidUriPart {
    pub kind: DidUriPartKind,
    pub start: u16,
    pub len: u8,
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DidUriPartKind {
    Scheme = 0,
    DidMethodName = 1,
    DidMethodSpecificId = 2,
    UserInfo = 3,
    Host = 4,
    Port = 5,
    Path = 6,
    Query = 7,
    Fragment = 8,
}

impl Display for DidUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl DidUri {

    pub fn part_string(&self, part: &DidUriPart) -> String {
        let r1 = usize::from(part.start);
        let r2 = r1 + usize::from(part.len);
        self.text[r1..r2].to_string()
    }

    pub fn did_method(&self) -> Option<String> {
        if let Some(part) = self.parts.get(1) {
            if part.kind == DidUriPartKind::DidMethodName {
                return Some(self.part_string(part));
            }
        }
        None
    }

    pub fn did_method_specific_id(&self) -> Option<String> {
        if let Some(part) = self.parts.get(1) {
            if part.kind == DidUriPartKind::DidMethodSpecificId {
                return Some(self.part_string(part));
            }
        }
        None
    }

    pub fn get_did(&self) -> Option<DidUri> {
        // Scheme:DidMethodName:DidMethodSpecificId
        match (self.parts.get(0), self.parts.get(1), self.parts.get(2)) {
            (Some(scheme), Some(method), Some(id)) => {
                let text = format!("{}:{}:{}", self.part_string(scheme), self.part_string(method), self.part_string(id));
                Some(
                    DidUri { 
                        text, 
                        parts: vec![scheme.clone(), method.clone(), id.clone()],
                    }
                )
            },
            _ => None,
        }
    }

    pub fn get_path_segments(&self) -> impl Iterator<Item = &DidUriPart> {
        self.parts
            .iter()
            .filter_map(
                |p| {
                    match p.kind {
                        DidUriPartKind::Path => Some(p),
                        _ => None,
                    }
                } 
            )
    }

    pub fn is_did(&self) -> bool {
        if let Some(part) = self.parts.get(0) {
            if part.kind == DidUriPartKind::Scheme {
                let r1 = usize::from(part.start);
                let r2 = r1 + usize::from(part.len);
                return self.text[r1..r2].eq("did");
            }
        }
        false
    }

    pub fn is_did_url(&self) -> bool {
        self.is_did() && self.parts.len() > 3
    }

    pub fn is_relative(&self) -> bool {
        if let Some(part) = self.parts.get(0) {
            part.kind != DidUriPartKind::Scheme
        } else {
            true
        }
    }
}

impl FromStr for DidUri {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_did_uri_reference(s.to_string())
    }
}

#[derive(Clone, Debug)]
struct ParseState<'a> {
    chars: Chars<'a>,
    c: char,
    pos: u16,
    uri: DidUri,
}

impl<'a> ParseState<'a> {
    fn new(input: &'a str) -> ParseState<'a> {
        ParseState { 
            chars: input.chars(), 
            c: '\0',
            pos: 0,
            uri: DidUri { 
                text: String::new(), 
                parts: Vec::new() 
            } 
        }
    }
}

fn parse_next_char<'a>(state: &mut ParseState<'a>, allow_pct: bool) -> Result<bool, String> {
    assert!(state.pos == 0 || state.c != '\0');
    if let Some(nc) = state.chars.next() {
        state.c = nc;
        state.pos += 1;
        if nc == '%' {
            if allow_pct {
                let mut ok = false;
                if let Some(hex1) = state.chars.next() {
                    if let Some(hex2) = state.chars.next() {
                        if let (Some(h1), Some(h2)) = (hex1.to_digit(16), hex2.to_digit(16)) {
                            if let Some(pc) = char::from_u32((h1 << 4) | h2) {
                                state.uri.text.push(pc);
                                ok = true;
                            }
                        }
                    }
                }
                if !ok {
                    return Err("pct-encoded error".to_string());
                }
            } else {
                return Err("pct-encoded not allowed".to_string());
            }
        } else {
            state.uri.text.push(nc);
        }
        Ok(true)
    } else {
        state.pos += 1;
        state.c = '\0';
        Ok(false)
    }
}

fn parse_did_url<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // did-url = did path-abempty [ "?" query ] [ "#" fragment ]
    if parse_did(state)? {
        if parse_path_abempty(state)? {
            if state.c == '?' {
                parse_next_char(state, true)?;
                parse_query(state)?;
            }
            if state.c == '#' {
                parse_next_char(state, true)?;
                parse_fragment(state)?;
            }
            return Ok(state.c == '\0');
        }
    }
    Ok(false)
}

fn parse_did<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // did                = "did:" method-name ":" method-specific-id
    let scheme_start = state.pos - 1;
    if state.c == 'd' && parse_next_char(state, false)? &&
        state.c == 'i' && parse_next_char(state, false)? &&
        state.c == 'd' && parse_next_char(state, false)? &&
        state.c == ':' {

        let part = DidUriPart { 
            kind: DidUriPartKind::Scheme, 
            start: scheme_start, 
            len: (state.pos - scheme_start - 1) as u8,
        };
        state.uri.parts.push(part);
        
        parse_next_char(state, false)?;
        if parse_did_method_name(state)? {
            if state.c == ':' {
                parse_next_char(state, true)?;
                return parse_did_method_specific_id(state);
            }
        }
    }
    Ok(false)
}

fn parse_did_method_name<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // method-name        = 1*method-char
    if parse_is_method_char(state.c) {
        let method_name_start= state.pos - 1;
        parse_next_char(state, false)?;
        loop {
            if parse_is_method_char(state.c) {
                parse_next_char(state, false)?;
            } else {
                break;
            }
        }
        let part = DidUriPart { 
            kind: DidUriPartKind::DidMethodName, 
            start: method_name_start, 
            len: (state.pos - method_name_start - 1) as u8,
        };
        state.uri.parts.push(part);
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_did_method_specific_id<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // method-specific-id = *( *idchar ":" ) 1*idchar
    let mut ends_with_idchar = false;
    let method_specific_id_start = state.pos - 1;
    loop {
        if parse_is_idchar(state.c) {
            ends_with_idchar = true;
            parse_next_char(state, true)?;
        } else if state.c == ':' {
            ends_with_idchar = false;
            parse_next_char(state, true)?;
        } else {
            break;
        }
    }
    if ends_with_idchar {
        let part = DidUriPart { 
            kind: DidUriPartKind::DidMethodSpecificId, 
            start: method_specific_id_start, 
            len: (state.pos - method_specific_id_start - 1) as u8,
        };
        state.uri.parts.push(part);
    }
    Ok(ends_with_idchar)
}

fn parse_is_method_char(c: char) -> bool {
    // method-char        = %x61-7A / DIGIT
    ('\x61' <= c && c <= '\x7A') || parse_is_digit(c)
}

fn parse_is_idchar(c: char) -> bool {
    // idchar             = ALPHA / DIGIT / "." / "-" / "_" / pct-encoded
    parse_is_alpha(c) || parse_is_digit(c) || (c == '.') || (c == '-') || (c == '_') || (c == '%')
}

fn parse_uri<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // URI           = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
    if parse_scheme(state)? {
        if state.c == ':' {
            parse_next_char(state, true)?;
            if parse_hier_part(state)? {
                if state.c == '?' {
                    parse_next_char(state, true)?;
                    parse_query(state)?;
                }
                if state.c == '#' {
                    parse_next_char(state, true)?;
                    parse_fragment(state)?;
                }
                return Ok(state.c == '\0');
            }
        }
    }
    Ok(false)
}

fn parse_hier_part<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // hier-part     = "//" authority path-abempty
    //               / path-absolute
    //               / path-rootless
    //               / path-empty
    match state.c {
        '/' => {
            // "//" authority path-abempty
            // / path-absolute
            parse_next_char(state, true)?;
            if state.c == '/' {
                // authority path-abempty
                parse_next_char(state, true)?;
                parse_authority(state)?;
                parse_path_abempty(state)
            } else {
                parse_path_absolute(state)
            }
        },
        '\0' => {
            // path-empty
            return Ok(true)
        },
        _ => {
            parse_path_rootless(state)
        },
    }
}

pub fn parse_did_uri_reference(input: String) -> Result<DidUri, String> {
    // URI-reference = DID / URI / relative-ref
    let mut state = ParseState::new(&input);
    parse_next_char(&mut state, false)?;
    let mut did_state = state.clone();
    if parse_did_url(&mut did_state)? {
        Ok(did_state.uri)
    } else {
        let mut uri_state = state.clone();
        if parse_uri(&mut uri_state)? {
            Ok(uri_state.uri)
        } else if parse_relative_ref(&mut state)? {
            // relative-ref
            Ok(state.uri)
        } else {
            Err("expected URI or relative-ref".to_string())
        }
    }
}

fn parse_relative_ref<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // relative-ref  = relative-part [ "?" query ] [ "#" fragment ]
    if parse_relative_part(state)? {
        if state.c == '?' {
            parse_next_char(state, true)?;
            parse_query(state)?;
        }
        if state.c == '#' {
            parse_next_char(state, true)?;
            parse_fragment(state)?;
        }
        return Ok(state.c == '\0');
    } else {
        Ok(false)
    }
}

fn parse_relative_part<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // relative-part = "//" authority path-abempty
    //                 / path-absolute
    //                 / path-noscheme
    //                 / path-empty
    if state.c == '/' {
        parse_next_char(state, false)?;
        if state.c == '/' {
            parse_next_char(state, false)?;
            parse_authority(state)?;
            parse_path_abempty(state)
        } else {
            parse_path_absolute(state)
        }
    } else if state.c == '\0' {
        // path-empty
        Ok(true)
    } else {
        parse_path_noscheme(state)
    }
}

fn parse_scheme<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // scheme        = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
    if parse_is_alpha(state.c) {
        let scheme_start = state.pos - 1;
        loop {
            parse_next_char(state, false)?;
            if !(parse_is_alpha(state.c) || 
                parse_is_digit(state.c) || 
                (state.c == '+') || 
                (state.c == '-') || 
                (state.c == '.')) {
                break;
            }
        }
        let part = DidUriPart { 
            kind: DidUriPartKind::Scheme, 
            start: scheme_start, 
            len: (state.pos - scheme_start - 1) as u8,
        };
        state.uri.parts.push(part);
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_authority<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // authority     = [ userinfo "@" ] host [ ":" port ]
    let mut userinfo_state = state.clone();
    let userinfo_start = userinfo_state.pos - 1;
    parse_userinfo(&mut userinfo_state)?;
    if userinfo_state.c == '@' {
        let part = DidUriPart { 
            kind: DidUriPartKind::UserInfo, 
            start: userinfo_start, 
            len: (userinfo_state.pos - userinfo_start) as u8,
        };
        userinfo_state.uri.parts.push(part);

        parse_next_char(&mut userinfo_state, true)?;
        let host_start = userinfo_state.pos - 1;
        if parse_host(&mut userinfo_state)? {
            let part = DidUriPart { 
                kind: DidUriPartKind::Host, 
                start: host_start, 
                len: (userinfo_state.pos - host_start) as u8,
            };
            userinfo_state.uri.parts.push(part);

            if userinfo_state.c == ':' {
                parse_next_char(&mut userinfo_state, false)?;
                let port_start = userinfo_state.pos - 1;
                parse_port(&mut userinfo_state)?;
                let part = DidUriPart { 
                    kind: DidUriPartKind::Port, 
                    start: port_start, 
                    len: (userinfo_state.pos - port_start) as u8,
                };
                userinfo_state.uri.parts.push(part);
            }
            state.clone_from(&userinfo_state);
            return Ok(true)
        }
    } else {
        let host_start = state.pos - 1;
        if parse_host(state)? {
            let part = DidUriPart { 
                kind: DidUriPartKind::Host, 
                start: host_start, 
                len: (state.pos - host_start - 1) as u8,
            };
            state.uri.parts.push(part);
            if state.c == ':' {
                parse_next_char(state, false)?;
                let port_start = state.pos - 1;
                parse_port(state)?;
                let part = DidUriPart { 
                    kind: DidUriPartKind::Port, 
                    start: port_start, 
                    len: (state.pos - port_start - 1) as u8,
                };
                state.uri.parts.push(part);
            }
            return Ok(true)
        }
    }
    Ok(false)
}

fn parse_userinfo<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // userinfo      = *( unreserved / pct-encoded / sub-delims / ":" )
    loop {
        if parse_is_unreserved(state.c) || (state.c == '%') || parse_is_sub_delims(state.c) || (state.c == ':') {
            parse_next_char(state, true)?;
        } else {
            return Ok(true)
        }
    }
}

fn parse_host<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // host          = IP-literal / IPv4address / reg-name
    let mut ip_literal_state = state.clone();
    if parse_ip_literal(&mut ip_literal_state)? {
        state.clone_from(&ip_literal_state);
        return Ok(true);
    }
    let mut ipv4_state = state.clone();
    if parse_ipv4address(&mut ipv4_state)? {
        state.clone_from(&ipv4_state);
        return Ok(true);
    }
    parse_reg_name(state)
}

fn parse_port<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // port          = *DIGIT
    loop {
        if parse_is_digit(state.c) {
            parse_next_char(state, false)?;
        } else {
            return Ok(true)
        }
    }
}

fn parse_ip_literal<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // IP-literal    = "[" ( IPv6address / IPvFuture  ) "]"
    if state.c == '[' {
        parse_next_char(state, false)?;
        let mut ipv6_state = state.clone();
        if parse_ipv6address(&mut ipv6_state)? {
            if ipv6_state.c == ']' {
                parse_next_char(&mut ipv6_state, false)?;
                state.clone_from(&ipv6_state);
                return Ok(true);
            }
        } else if parse_ipvfuture(state)? {
            if state.c == ']' {
                parse_next_char(state, false)?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn parse_ipvfuture<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // IPvFuture     = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
    if state.c == 'v' {
        parse_next_char(state, false)?;
        if parse_is_hex_digit(state.c) {
            parse_next_char(state, false)?;
            loop {
                if parse_is_hex_digit(state.c) {
                    parse_next_char(state, false)?;
                } else {
                    break;
                }
            }
            if state.c == '.' {
                parse_next_char(state, false)?;
                if parse_is_unreserved(state.c) || parse_is_sub_delims(state.c) || (state.c == ':') {
                    parse_next_char(state, false)?;
                    loop {
                        if parse_is_unreserved(state.c) || parse_is_sub_delims(state.c) || (state.c == ':') {
                            parse_next_char(state, false)?;
                        } else {
                            break;
                        }
                    }
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

fn parse_ipv6address<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // IPv6address   =                            6( h16 ":" ) ls32     variant = 1
    //               /                       "::" 5( h16 ":" ) ls32     variant = 2
    //               / [               h16 ] "::" 4( h16 ":" ) ls32     variant = 3
    //               / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32     variant = 4
    //               / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32     variant = 5
    //               / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32     variant = 6
    //               / [ *4( h16 ":" ) h16 ] "::"              ls32     variant = 7
    //               / [ *5( h16 ":" ) h16 ] "::"              h16      variant = 8
    //               / [ *6( h16 ":" ) h16 ] "::"                       variant = 9

    'variant_loop: for variant in 1..=9 {
        let mut variant_state = state.clone();
        let left;
        let right;
       
        match variant {
            1 => { left = 0; right = 6; },
            2 => { left = 0; right = 5; },
            3 => { left = 1; right = 4; },
            4 => { left = 2; right = 3; },
            5 => { left = 3; right = 2; },
            6 => { left = 4; right = 1; },
            7 => { left = 5; right = 0; },
            8 => { left = 6; right = 0; },
            9 => { left = 7; right = 0; },
            _ => {
                unreachable!();
            }
        }

        if variant > 1 {
            let mut i = 0;
            loop {
                if i < left && parse_h16(&mut variant_state)? {
                    i += 1;
                    if variant_state.c == ':' {
                        parse_next_char(&mut variant_state, false)?;
                    } else {
                        continue 'variant_loop;
                    }
                } else {
                    break;
                }
            }
            if i == 0 {
                if variant_state.c == ':' {
                    parse_next_char(&mut variant_state, false)?;
                } else {
                    continue 'variant_loop;
                }
            }
            if variant_state.c == ':' {
                parse_next_char(&mut variant_state, false)?;
            } else {
                continue 'variant_loop;
            }
        }

        for _ in 0..right {
            if parse_h16(&mut variant_state)? {
                if variant_state.c == ':' {
                    parse_next_char(&mut variant_state, false)?;
                } else {
                    continue 'variant_loop;
                }
            } else {
                continue 'variant_loop;
            }
        }
        if variant <= 7 && parse_ls32(&mut variant_state)? {
            state.clone_from(&variant_state);
            return Ok(true);
        } else if variant == 8 && parse_h16(&mut variant_state)? {
            state.clone_from(&variant_state);
            return Ok(true);
        } else if variant == 9 {
            state.clone_from(&variant_state);
            return Ok(true);
        }
    }
    Ok(false)
}

fn parse_h16<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // h16           = 1*4HEXDIG
    if parse_is_hex_digit(state.c) {
        parse_next_char(state, false)?;
        for _ in 0..3 {
            if parse_is_hex_digit(state.c) {
                parse_next_char(state, false)?;
            } else {
                return Ok(true);
            }
        }
        return Ok(true);
    }
    Ok(false)
}

fn parse_ls32<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // ls32          = ( h16 ":" h16 ) / IPv4address
    let mut left_state = state.clone();
    if parse_h16(&mut left_state)? {
        if left_state.c == ':' {
            parse_next_char(&mut left_state, false)?;
            if parse_h16(&mut left_state)? {
                state.clone_from(&left_state);
                return Ok(true);
            }
        }
    } else {
        return parse_ipv4address(state);
    }
    Ok(false)
}

fn parse_ipv4address<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // IPv4address   = dec-octet "." dec-octet "." dec-octet "." dec-octet
    for _ in 0..3 {
        if parse_dec_octet(state)? {
            if state.c == '.' {
                parse_next_char(state, false)?;
            } else {
                return Ok(false);
            }
        }
    }
    parse_dec_octet(state)
}

fn parse_dec_octet<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // dec-octet     = DIGIT                 ; 0-9
    //               / %x31-39 DIGIT         ; 10-99
    //               / "1" 2DIGIT            ; 100-199
    //               / "2" %x30-34 DIGIT     ; 200-249
    //               / "25" %x30-35          ; 250-255
    if state.c == '2' {
        parse_next_char(state, false)?;
        if state.c == '5' {
            parse_next_char(state, false)?;
            if '\x30' <= state.c && state.c <= '\x35' {
                parse_next_char(state, false)?;
            }
        } else if '\x30' <= state.c && state.c <= '\x34' {
            parse_next_char(state, false)?;
            if parse_is_digit(state.c) {
                parse_next_char(state, false)?;
            }
        }
        return Ok(true);
    } else if state.c == '1' {
        parse_next_char(state, false)?;
        if parse_is_digit(state.c) {
            parse_next_char(state, false)?;
            if parse_is_digit(state.c) {
                parse_next_char(state, false)?;
            }
        }
        return Ok(true);
    } else if '\x31' <= state.c && state.c <= '\x39' {
        parse_next_char(state, false)?;
        if parse_is_digit(state.c) {
            parse_next_char(state, false)?;
        }
        return Ok(true);
    } else if parse_is_digit(state.c) {
        parse_next_char(state, false)?;
        return Ok(true);
    }
    Ok(false)
}

fn parse_reg_name<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // reg-name      = *( unreserved / pct-encoded / sub-delims )
    loop {
        if parse_is_unreserved(state.c) || (state.c == '%') || parse_is_sub_delims(state.c) {
            parse_next_char(state, true)?;
        } else {
            return Ok(true);
        }
    }
}

// path          = path-abempty    ; begins with "/" or is empty
//               / path-absolute   ; begins with "/" but not "//"
//               / path-noscheme   ; begins with a non-colon segment
//               / path-rootless   ; begins with a segment
//               / path-empty      ; zero characters

fn parse_path_abempty<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // path-abempty  = *( "/" segment )
    loop {
        match state.c {
            '/' => {
                let path_start = state.pos - 1;
                parse_next_char(state, true)?;
                parse_segment(state)?;
                let part = DidUriPart { 
                    kind: DidUriPartKind::Path, 
                    start: path_start, 
                    len: (state.pos - path_start - 1) as u8,
                };
                state.uri.parts.push(part);
            },
            _ => {
                return Ok(true);
            }
        }
    }
}

fn parse_path_absolute<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // path-absolute = "/" [ segment-nz *( "/" segment ) ]
    let mut path_start = state.pos - 2;
    if parse_segment_nz(state)? {
        let part = DidUriPart { 
            kind: DidUriPartKind::Path, 
            start: path_start, 
            len: (state.pos - path_start - 1) as u8,
        };
        state.uri.parts.push(part);
        loop {
            if state.c == '/' {
                path_start = state.pos - 1;
                parse_next_char(state, true)?;
                parse_segment(state)?;
                let part = DidUriPart { 
                    kind: DidUriPartKind::Path, 
                    start: path_start, 
                    len: (state.pos - path_start - 1) as u8,
                };
                state.uri.parts.push(part);
            } else {
                break;
            }
        }
    }
    Ok(true)
}


fn parse_path_noscheme<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // path-noscheme = segment-nz-nc *( "/" segment )
    let mut path_start = state.pos - 1;
    if parse_segment_nz_nc(state)? {
        let part = DidUriPart { 
            kind: DidUriPartKind::Path, 
            start: path_start, 
            len: (state.pos - path_start - 1) as u8,
        };
        state.uri.parts.push(part);
        loop {
            if state.c == '/' {
                path_start = state.pos - 1;
                parse_next_char(state, true)?;
                parse_segment(state)?;
                let part = DidUriPart { 
                    kind: DidUriPartKind::Path, 
                    start: path_start, 
                    len: (state.pos - path_start - 1) as u8,
                };
                state.uri.parts.push(part);
            } else {
                break;
            }
        }
        return Ok(true);
    }
    Ok(false)
}

fn parse_path_rootless<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // path-rootless = segment-nz *( "/" segment )
    let mut path_start = state.pos - 1;
    if parse_segment_nz(state)? {
        let part = DidUriPart { 
            kind: DidUriPartKind::Path, 
            start: path_start, 
            len: (state.pos - path_start - 1) as u8,
        };
        state.uri.parts.push(part);
        loop {
            if state.c == '/' {
                path_start = state.pos - 1;
                parse_next_char(state, true)?;
                parse_segment(state)?;
                let part = DidUriPart { 
                    kind: DidUriPartKind::Path, 
                    start: path_start, 
                    len: (state.pos - path_start - 1) as u8,
                };
                state.uri.parts.push(part);
            } else {
                return Ok(true);
            }
        }
    } else {
        Ok(false)
    }
}

// path-empty    = 0<pchar>

fn parse_segment<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // segment       = *pchar
    loop {
        if parse_is_pchar(state.c) {
            parse_next_char(state, true)?;
        } else {
            return Ok(true);
        }
    }
}

fn parse_segment_nz<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // segment-nz    = 1*pchar
    if parse_is_pchar(state.c) {
        loop {
            parse_next_char(state, true)?;
            if !parse_is_pchar(state.c) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn parse_segment_nz_nc<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
    //                 ; non-zero-length segment without any colon ":"
    if parse_is_unreserved(state.c) || (state.c == '%') || parse_is_sub_delims(state.c) || (state.c == '@') {
        loop {
            parse_next_char(state, true)?;
            if !(parse_is_unreserved(state.c) || (state.c == '%') || parse_is_sub_delims(state.c) || (state.c == '@')) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}


fn parse_is_pchar(c: char) -> bool {
    // pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
    parse_is_unreserved(c) || (c == '%') || parse_is_sub_delims(c) || (c == ':') || (c == '@')
}


fn parse_query<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // query         = *( pchar / "/" / "?" )
    let query_start = state.pos - 1;
    loop {
        if parse_is_pchar(state.c) || (state.c == '/') || (state.c == '?') {
            parse_next_char(state, true)?;
        } else {
            let part = DidUriPart { 
                kind: DidUriPartKind::Query, 
                start: query_start, 
                len: (state.pos - query_start - 1) as u8,
            };
            state.uri.parts.push(part);
            return Ok(true);
        }
    }
}

fn parse_fragment<'a>(state: &mut ParseState<'a>) -> Result<bool, String> {
    // fragment      = *( pchar / "/" / "?" )
    let fragment_start = state.pos - 1;
    loop {
        if parse_is_pchar(state.c) || (state.c == '/') || (state.c == '?') {
            parse_next_char(state, true)?;
        } else {
            let part = DidUriPart { 
                kind: DidUriPartKind::Fragment, 
                start: fragment_start, 
                len: (state.pos - fragment_start - 1) as u8,
            };
            state.uri.parts.push(part);
            return Ok(true);
        }
    }
}

// pct-encoded   = "%" HEXDIG HEXDIG

fn parse_is_unreserved(c: char) -> bool {
    // unreserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
    parse_is_alpha(c) || parse_is_digit(c) || (c == '-') || (c == '.') || (c == '_') || (c == '~')
}

// reserved      = gen-delims / sub-delims

// gen-delims    = ":" / "/" / "?" / "#" / "[" / "]" / "@"

fn parse_is_sub_delims(c: char) -> bool {
    // sub-delims    = "!" / "$" / "&" / "'" / "(" / ")"
    //               / "*" / "+" / "," / ";" / "="
    (c == '!') || (c == '$') || (c == '&') || (c == '\'') || (c == '(') || (c == ')') ||
    (c == '*') || (c == '+') || (c == ',') || (c == ';') || (c == '=')
}

fn parse_is_alpha(c: char) -> bool {
    ('\x41' <= c && c <= '\x5A') || ('\x61' <= c && c <= '\x7A') // ALPHA
}

fn parse_is_digit(c: char) -> bool {
    '\x30' <= c && c <= '\x39' // DIGIT
}

fn parse_is_hex_digit(c: char) -> bool {
    parse_is_digit(c) || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _print_uri(uri: &DidUri) {
        println!("text: \"{}\"", uri.text);
        for part in uri.parts.iter() {
            let r1 = usize::from(part.start);
            let r2 = r1 + usize::from(part.len);
            let s = uri.text[r1..r2].to_string();
            println!("part: \"{s}\" {part:?}");
        }
    }

    #[test]
    fn test_uri_did() {
        let did = "did:policy:e%62xample:12345/aaa//b?x=2&y=1#f32".parse();
        let expected = 
            DidUri {
                text: "did:policy:ebxample:12345/aaa//b?x=2&y=1#f32".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 3},
                    DidUriPart { kind: DidUriPartKind::DidMethodName, start: 4, len: 6},
                    DidUriPart { kind: DidUriPartKind::DidMethodSpecificId, start: 11, len: 14},
                    DidUriPart { kind: DidUriPartKind::Path, start: 25, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 29, len: 1 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 30, len: 2 },
                    DidUriPart { kind: DidUriPartKind::Query, start: 33, len: 7 },
                    DidUriPart { kind: DidUriPartKind::Fragment, start: 41, len: 3 },
                ],
            };
        assert_eq!(did, Ok(expected));
    }    

    #[test]
    fn test_uri_http_localhost() {
        let uri: Result<DidUri, String> = "http://localhost/".parse();
        let expected = 
            DidUri {
                text: "http://localhost/".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 7, len: 9 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 16, len: 1 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }    

    #[test]
    fn test_uri_http_example_com() {
        let uri: Result<DidUri, String> = "http://example.com/".parse();
        let expected = 
            DidUri {
                text: "http://example.com/".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 7, len: 11 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 18, len: 1 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_file() {
        let uri: Result<DidUri, String> = "file:///etc/hosts".parse();
        let expected = 
            DidUri {
                text: "file:///etc/hosts".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 7, len: 0 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 7, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 11, len: 6 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_ftp() {
        let uri: Result<DidUri, String> = "ftp://ftp.is.co.za/rfc/rfc1808.txt".parse();
        let expected = 
            DidUri {
                text: "ftp://ftp.is.co.za/rfc/rfc1808.txt".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 3 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 6, len: 12 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 18, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 22, len: 12 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_ietf() {
        let uri: Result<DidUri, String> = "http://www.ietf.org/rfc/rfc2396.txt".parse();
        let expected = 
            DidUri {
                text: "http://www.ietf.org/rfc/rfc2396.txt".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 7, len: 12 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 19, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 23, len: 12 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_ipv6_variant_1() {
        // 6( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("1abc:200:31:4:5:6:7:8");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "1abc:200:31:4:5:6:7:8".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_2() {
        // "::" 5( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("::1:21:33:4aa:5:6F:7A");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "::1:21:33:4aa:5:6F:7A".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_3a() {
        // [               h16 ] "::" 4( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("::1:21:33:4aa:6F:7A");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "::1:21:33:4aa:6F:7A".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_3b() {
        // [               h16 ] "::" 4( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("a000::1:21:33:4aa:6F:7A");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "a000::1:21:33:4aa:6F:7A".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_4a() {
        // [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("b11::1:21:33:AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "b11::1:21:33:AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_4b() {
        // [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("a000:b11::1:21:33:AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "a000:b11::1:21:33:AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_5a() {
        // [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("b11::1:21:AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "b11::1:21:AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_5b() {
        // [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("a2:b11::1:21:AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "a2:b11::1:21:AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_5c() {
        // [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("a2:b11:c3::1:21:AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "a2:b11:c3::1:21:AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_6() {
        // [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("a:b:c:d::111:AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "a:b:c:d::111:AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_7() {
        // [ *4( h16 ":" ) h16 ] "::"              ls32
        // ls32          = ( h16 ":" h16 ) / IPv4address
        let mut state = ParseState::new("a:b:c:d:e::AA:BB");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "a:b:c:d:e::AA:BB".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_8() {
        // [ *5( h16 ":" ) h16 ] "::"              h16
        let mut state = ParseState::new("2001:db8::7");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "2001:db8::7".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ipv6_variant_9() {
        // [ *6( h16 ":" ) h16 ] "::"
        let mut state = ParseState::new("2001::");
        parse_next_char(&mut state, false).unwrap();
        let result = parse_ipv6address(&mut state);
        let expected = 
            DidUri {
                text: "2001::".to_string(),
                parts: vec![],
            };
        assert_eq!(Ok(true), result);
        assert_eq!(state.uri, expected);
    }

    #[test]
    fn test_uri_ldap() {
        let uri: Result<DidUri, String> = "ldap://[2001:db8::7]/c=GB?objectClass?one".parse();
        let expected = 
            DidUri {
                text: "ldap://[2001:db8::7]/c=GB?objectClass?one".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 7, len: 13 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 20, len: 5 },
                    DidUriPart { kind: DidUriPartKind::Query, start: 26, len: 15 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_mailto() {
        let uri: Result<DidUri, String> = "mailto:John.Doe@example.com".parse();
        let expected = 
            DidUri {
                text: "mailto:John.Doe@example.com".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 6 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 7, len: 20 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_news() {
        let uri: Result<DidUri, String> = "news:comp.infosystems.www.servers.unix".parse();
        let expected = 
            DidUri {
                text: "news:comp.infosystems.www.servers.unix".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 5, len: 33 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_tel() {
        let uri: Result<DidUri, String> = "tel:+1-816-555-1212".parse();
        let expected = 
            DidUri {
                text: "tel:+1-816-555-1212".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 3 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 4, len: 15 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_telnet() {
        let uri: Result<DidUri, String> = "telnet://192.0.2.16:80/".parse();
        let expected = 
            DidUri {
                text: "telnet://192.0.2.16:80/".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 6 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 9, len: 10 },
                    DidUriPart { kind: DidUriPartKind::Port, start: 20, len: 2 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 22, len: 1 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_docbook() {
        let uri: Result<DidUri, String> = "urn:oasis:names:specification:docbook:dtd:xml:4.1.2".parse();
        let expected = 
            DidUri {
                text: "urn:oasis:names:specification:docbook:dtd:xml:4.1.2".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 3 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 4, len: 47 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_foo() {
        let uri: Result<DidUri, String> = "foo://example.com:8042/over/there?name=ferret#nose".parse();
        let expected = 
            DidUri {
                text: "foo://example.com:8042/over/there?name=ferret#nose".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 3 },
                    DidUriPart { kind: DidUriPartKind::Host, start: 6, len: 11 },
                    DidUriPart { kind: DidUriPartKind::Port, start: 18, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 22, len: 5 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 27, len: 6 },
                    DidUriPart { kind: DidUriPartKind::Query, start: 34, len: 11 },
                    DidUriPart { kind: DidUriPartKind::Fragment, start: 46, len: 4 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_uri_urn() {
        let uri: Result<DidUri, String> = "urn:example:animal:ferret:nose".parse();
        let expected = 
            DidUri {
                text: "urn:example:animal:ferret:nose".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Scheme, start: 0, len: 3 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 4, len: 26 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }

    #[test]
    fn test_relative_ref() {
        let uri: Result<DidUri, String> = "/aaa/bbb?x=2#f11".parse();
        let expected = 
            DidUri {
                text: "/aaa/bbb?x=2#f11".to_string(),
                parts: vec![
                    DidUriPart { kind: DidUriPartKind::Path, start: 0, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Path, start: 4, len: 4 },
                    DidUriPart { kind: DidUriPartKind::Query, start: 9, len: 3 },
                    DidUriPart { kind: DidUriPartKind::Fragment, start: 13, len: 3 },
                ],
            };
        assert_eq!(uri, Ok(expected));
    }



}
