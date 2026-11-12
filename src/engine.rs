//! Renaming engine: rules, previews, validation, presets.
//! There is no cap on the number of rules or presets, and presets have no folder restrictions.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub enum Kind {
    #[default]
    Insert,
    Delete,
    Remove,
    Replace,
    Extension,
    Strip,
    Case,
    Number,
    Numbering,
    Randomize,
    CleanUp,
    Regex,
    Rearrange,
    Padding,
    Transliterate,
    AssignNames,
    MapNames,
}

impl Kind {
    pub const ALL: [Kind; 17] = [
        Kind::Insert,
        Kind::Delete,
        Kind::Remove,
        Kind::Replace,
        Kind::Extension,
        Kind::Strip,
        Kind::Case,
        Kind::Number,
        Kind::Numbering,
        Kind::Randomize,
        Kind::CleanUp,
        Kind::Regex,
        Kind::Rearrange,
        Kind::Padding,
        Kind::Transliterate,
        Kind::AssignNames,
        Kind::MapNames,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Kind::Insert => "Insert",
            Kind::Delete => "Delete",
            Kind::Remove => "Remove",
            Kind::Replace => "Replace",
            Kind::Extension => "Extension",
            Kind::Strip => "Strip",
            Kind::Case => "Case",
            Kind::Number => "Serialize",
            Kind::Numbering => "Numbering",
            Kind::Randomize => "Randomize",
            Kind::CleanUp => "Clean up",
            Kind::Regex => "Regular expression",
            Kind::Rearrange => "Rearrange",
            Kind::Padding => "Padding",
            Kind::Transliterate => "Transliterate",
            Kind::AssignNames => "Names from text lines",
            Kind::MapNames => "Map old to new names",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Rule {
    pub kind: Kind,
    pub enabled: bool,
    pub skip_ext: bool,
    pub text1: String,
    pub text2: String,
    pub n1: i64,
    pub n2: i64,
    pub n3: i64,
    pub choice: usize,
    pub flag1: bool,
    pub flag2: bool,
    pub flag3: bool,
    pub flag4: bool,
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            kind: Kind::Insert,
            enabled: true,
            skip_ext: true,
            text1: String::new(),
            text2: String::new(),
            n1: 0,
            n2: 0,
            n3: 0,
            choice: 0,
            flag1: false,
            flag2: false,
            flag3: false,
            flag4: false,
        }
    }
}

impl Rule {
    pub fn new(kind: Kind) -> Rule {
        let mut r = Rule::default();
        r.kind = kind;
        match kind {
            Kind::Delete => {
                r.n1 = 0;
                r.n2 = 1;
            }
            Kind::Replace => {
                r.choice = 1;
            }
            Kind::Remove => {
                r.choice = 1;
            }
            Kind::Extension => {
                r.choice = 1;
            }
            Kind::Strip => {
                r.flag1 = true;
            }
            Kind::Number => {
                r.n1 = 1;
                r.n2 = 1;
                r.n3 = 2;
                r.choice = 1;
                r.text1 = "_".to_string();
            }
            Kind::Numbering => {
                // text1 = prefix, text2 = suffix, n1 = start, n2 = step, n3 = digits,
                // choice = placement (0 before, 1 after, 2 replace), flag1 = descending.
                r.n1 = 1;
                r.n2 = 1;
                r.n3 = 2;
                r.choice = 2;
            }
            Kind::Randomize => {
                r.n1 = 6;
                r.n2 = 1;
                r.choice = 1;
                r.flag1 = true;
                r.flag2 = true;
                r.text1 = "_".to_string();
            }
            Kind::CleanUp => {
                r.flag1 = true;
                r.flag3 = true;
            }
            Kind::Regex => {
                r.text1 = "(.*)".to_string();
                r.text2 = "$1".to_string();
                r.choice = 1;
            }
            Kind::Rearrange => {
                r.text1 = " -_".to_string();
                r.text2 = "{2} {1}".to_string();
            }
            Kind::Padding => {
                r.n1 = 3;
            }
            Kind::AssignNames | Kind::MapNames => {
                r.skip_ext = true;
            }
            _ => {}
        }
        r
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub rules: Vec<Rule>,
}

pub fn presets_path() -> PathBuf {
    match std::env::var("APPDATA") {
        Ok(dir) => {
            let mut p = PathBuf::from(dir);
            p.push("batch-renamer");
            let _ = std::fs::create_dir_all(&p);
            p.push("presets.json");
            p
        }
        Err(_) => PathBuf::from("presets.json"),
    }
}

pub fn load_presets() -> Vec<Preset> {
    let path = presets_path();
    match std::fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str::<Vec<Preset>>(&text) {
            Ok(presets) => presets,
            Err(_) => {
                // Do not let the next save silently overwrite a file we could not read.
                let _ = std::fs::rename(&path, path.with_extension("json.bak"));
                Vec::new()
            }
        },
        Err(_) => Vec::new(),
    }
}

pub fn save_presets(presets: &[Preset]) -> Result<(), String> {
    let text = serde_json::to_string_pretty(presets).map_err(|e| e.to_string())?;
    let path = presets_path();
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, &path).map_err(|e| e.to_string())
}

/// Splits "photo.final.jpg" into ("photo.final", "jpg"). A leading dot is not an extension.
pub fn split_name(file_name: &str) -> (String, String) {
    match file_name.rfind('.') {
        Some(p) if p > 0 => (file_name[..p].to_string(), file_name[p + 1..].to_string()),
        _ => (file_name.to_string(), String::new()),
    }
}

/// `index` is the position of the file among the marked files, `total` is how many files are marked.
pub fn apply_rules(
    rules: &[Rule],
    stem: &str,
    ext: &str,
    index: usize,
    total: usize,
) -> (String, String) {
    let mut stem = stem.to_string();
    let mut ext = ext.to_string();
    for r in rules {
        if !r.enabled {
            continue;
        }
        if r.kind == Kind::Extension {
            ext = ext_rule(r, &ext);
            continue;
        }
        if r.skip_ext || ext.is_empty() {
            stem = transform(r, &stem, index, total);
        } else {
            let full = format!("{}.{}", stem, ext);
            let out = transform(r, &full, index, total);
            match out.rfind('.') {
                Some(p) if p > 0 => {
                    stem = out[..p].to_string();
                    ext = out[p + 1..].to_string();
                }
                _ => {
                    stem = out;
                    ext = String::new();
                }
            }
        }
    }
    (stem, ext)
}

const RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

fn is_reserved_name(name: &str) -> bool {
    let base = name.split('.').next().unwrap_or("").trim_end().to_uppercase();
    RESERVED_NAMES.contains(&base.as_str())
}

pub fn invalid_name(name: &str) -> bool {
    name.is_empty()
        || is_reserved_name(name)
        || name.chars().any(|c| "\\/:*?\"<>|".contains(c) || (c as u32) < 32)
        || name.ends_with(' ')
        || name.ends_with('.')
}

fn ext_rule(r: &Rule, ext: &str) -> String {
    let t = r.text1.trim_start_matches('.').to_string();
    match r.choice {
        0 => t,
        1 => ext.to_lowercase(),
        2 => ext.to_uppercase(),
        3 => String::new(),
        _ => {
            if ext.is_empty() {
                t
            } else {
                ext.to_string()
            }
        }
    }
}

fn fold(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

fn replace_occ(s: &str, find: &str, with: &str, cs: bool, mode: usize) -> String {
    if find.is_empty() {
        return s.to_string();
    }
    let sc: Vec<char> = s.chars().collect();
    let fc: Vec<char> = find.chars().collect();
    let mut hits: Vec<usize> = Vec::new();
    let mut i = 0;
    while i + fc.len() <= sc.len() {
        let mut same = true;
        for k in 0..fc.len() {
            let a = sc[i + k];
            let b = fc[k];
            let eq = if cs { a == b } else { fold(a) == fold(b) };
            if !eq {
                same = false;
                break;
            }
        }
        if same {
            hits.push(i);
            i += fc.len();
        } else {
            i += 1;
        }
    }
    if hits.is_empty() {
        return s.to_string();
    }
    let chosen: Vec<usize> = match mode {
        0 => vec![hits[0]],
        2 => vec![hits[hits.len() - 1]],
        _ => hits,
    };
    let mut out = String::new();
    let mut i = 0;
    while i < sc.len() {
        if chosen.contains(&i) {
            out.push_str(with);
            i += fc.len();
        } else {
            out.push(sc[i]);
            i += 1;
        }
    }
    out
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn translit_char(c: char) -> Option<&'static str> {
    let s = match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' => "a",
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' => "A",
        'è' | 'é' | 'ê' | 'ë' | 'ē' => "e",
        'È' | 'É' | 'Ê' | 'Ë' | 'Ē' => "E",
        'ì' | 'í' | 'î' | 'ï' | 'ī' => "i",
        'Ì' | 'Í' | 'Î' | 'Ï' | 'Ī' => "I",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' => "o",
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' => "O",
        'ù' | 'ú' | 'û' | 'ü' | 'ū' => "u",
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ū' => "U",
        'ç' | 'ć' | 'č' => "c",
        'Ç' | 'Ć' | 'Č' => "C",
        'ñ' | 'ń' => "n",
        'Ñ' | 'Ń' => "N",
        'š' | 'ś' => "s",
        'Š' | 'Ś' => "S",
        'ž' | 'ź' | 'ż' => "z",
        'Ž' | 'Ź' | 'Ż' => "Z",
        'ý' | 'ÿ' => "y",
        'Ý' => "Y",
        'ł' => "l",
        'Ł' => "L",
        'ß' => "ss",
        'æ' => "ae",
        'Æ' => "AE",
        'œ' => "oe",
        'Œ' => "OE",
        _ => return None,
    };
    Some(s)
}

fn transform(r: &Rule, s: &str, index: usize, total: usize) -> String {
    match r.kind {
        Kind::Insert => {
            let chars: Vec<char> = s.chars().collect();
            let n = chars.len();
            match r.choice {
                0 => format!("{}{}", r.text1, s),
                1 => format!("{}{}", s, r.text1),
                _ => {
                    let pos = (r.n1.max(0) as usize).min(n);
                    let p = if r.flag1 { n - pos } else { pos };
                    let head: String = chars[..p].iter().collect();
                    let tail: String = chars[p..].iter().collect();
                    format!("{}{}{}", head, r.text1, tail)
                }
            }
        }
        Kind::Delete => {
            if r.choice == 0 {
                let chars: Vec<char> = s.chars().collect();
                let n = chars.len();
                let start = (r.n1.max(0) as usize).min(n);
                let count = r.n2.max(0) as usize;
                let (begin, end) = if r.flag1 {
                    let e = n - start;
                    (e.saturating_sub(count), e)
                } else {
                    (start, (start + count).min(n))
                };
                let head: String = chars[..begin].iter().collect();
                let tail: String = chars[end..].iter().collect();
                format!("{}{}", head, tail)
            } else {
                let from = if r.text1.is_empty() {
                    Some((0usize, 0usize))
                } else {
                    s.find(&r.text1).map(|i| (i, i + r.text1.len()))
                };
                match from {
                    None => s.to_string(),
                    Some((fs, fe)) => {
                        let until = if r.text2.is_empty() {
                            Some((s.len(), s.len()))
                        } else {
                            s[fe..].find(&r.text2).map(|i| (fe + i, fe + i + r.text2.len()))
                        };
                        match until {
                            None => s.to_string(),
                            Some((ts, te)) => {
                                let (a, b) = if r.flag1 { (fe, ts) } else { (fs, te) };
                                format!("{}{}", &s[..a], &s[b..])
                            }
                        }
                    }
                }
            }
        }
        Kind::Remove => replace_occ(s, &r.text1, "", r.flag1, r.choice),
        Kind::Replace => replace_occ(s, &r.text1, &r.text2, r.flag1, r.choice),
        Kind::Extension => s.to_string(),
        Kind::Strip => s
            .chars()
            .filter(|c| {
                let digit = r.flag1 && c.is_numeric();
                let letter = r.flag2 && c.is_alphabetic();
                let symbol = r.flag3 && !c.is_alphanumeric() && !c.is_whitespace();
                let custom = !r.text1.is_empty() && r.text1.contains(*c);
                !(digit || letter || symbol || custom)
            })
            .collect(),
        Kind::Case => match r.choice {
            0 => s.to_lowercase(),
            1 => s.to_uppercase(),
            2 => {
                let mut out = String::new();
                let mut new_word = true;
                for c in s.chars() {
                    if c.is_alphanumeric() {
                        if new_word {
                            out.extend(c.to_uppercase());
                        } else {
                            out.extend(c.to_lowercase());
                        }
                        new_word = false;
                    } else {
                        out.push(c);
                        new_word = true;
                    }
                }
                out
            }
            3 => {
                let mut out = String::new();
                for (i, c) in s.chars().enumerate() {
                    if i == 0 {
                        out.extend(c.to_uppercase());
                    } else {
                        out.extend(c.to_lowercase());
                    }
                }
                out
            }
            _ => s
                .chars()
                .map(|c| {
                    if c.is_uppercase() {
                        c.to_lowercase().next().unwrap_or(c)
                    } else {
                        c.to_uppercase().next().unwrap_or(c)
                    }
                })
                .collect(),
        },
        Kind::Number => {
            let val = r.n1.saturating_add(r.n2.saturating_mul(index as i64));
            let num = format!("{:0width$}", val, width = r.n3.clamp(0, 64) as usize);
            match r.choice {
                0 => format!("{}{}{}", num, r.text1, s),
                1 => format!("{}{}{}", s, r.text1, num),
                _ => num,
            }
        }
        Kind::Numbering => {
            // Ascending: the first file gets `start`. Descending: the first file gets the
            // highest number and the last one gets `start`.
            let step = r.n2.max(1);
            let position = if r.flag1 {
                total.saturating_sub(1).saturating_sub(index)
            } else {
                index
            };
            let val = r.n1.saturating_add(step.saturating_mul(position as i64));
            let num = format!("{:0width$}", val, width = r.n3.clamp(0, 64) as usize);
            let label = format!("{}{}{}", r.text1, num, r.text2);
            match r.choice {
                0 => format!("{}{}", label, s),
                1 => format!("{}{}", s, label),
                _ => label,
            }
        }
        Kind::Randomize => {
            let mut set: Vec<char> = Vec::new();
            if r.flag1 {
                set.extend("0123456789".chars());
            }
            if r.flag2 {
                set.extend("abcdefghijklmnopqrstuvwxyz".chars());
            }
            if r.flag3 {
                set.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
            }
            if set.is_empty() {
                set.extend("0123456789".chars());
            }
            let mut state: u64 = (r.n2 as u64)
                .wrapping_mul(0x1234_5678_9ABC_DEF1)
                .wrapping_add((index as u64).wrapping_mul(0x0DEF_ACED_1234_5679));
            let len = r.n1.clamp(1, 200) as usize;
            let mut rnd = String::new();
            for _ in 0..len {
                let k = (splitmix(&mut state) % (set.len() as u64)) as usize;
                rnd.push(set[k]);
            }
            match r.choice {
                0 => format!("{}{}{}", rnd, r.text1, s),
                1 => format!("{}{}{}", s, r.text1, rnd),
                _ => rnd,
            }
        }
        Kind::CleanUp => {
            let mut out = s.to_string();
            if r.flag4 {
                let mut tmp = String::new();
                let mut depth = 0i32;
                for c in out.chars() {
                    if c == '(' || c == '[' || c == '{' {
                        depth += 1;
                    } else if c == ')' || c == ']' || c == '}' {
                        if depth > 0 {
                            depth -= 1;
                        }
                    } else if depth == 0 {
                        tmp.push(c);
                    }
                }
                out = tmp;
            }
            if r.flag2 {
                out = out.replace('_', " ");
            }
            if r.flag3 {
                let mut tmp = String::new();
                let mut prev_space = false;
                for c in out.chars() {
                    if c == ' ' {
                        if !prev_space {
                            tmp.push(c);
                        }
                        prev_space = true;
                    } else {
                        tmp.push(c);
                        prev_space = false;
                    }
                }
                out = tmp;
            }
            if r.flag1 {
                out = out.trim().to_string();
            }
            out
        }
        Kind::Regex => {
            let pattern = if r.flag2 {
                format!("(?i){}", r.text1)
            } else {
                r.text1.clone()
            };
            match Regex::new(&pattern) {
                Ok(re) => {
                    if r.choice == 0 {
                        re.replacen(s, 1, r.text2.as_str()).to_string()
                    } else {
                        re.replace_all(s, r.text2.as_str()).to_string()
                    }
                }
                Err(_) => s.to_string(),
            }
        }
        Kind::Rearrange => {
            let parts: Vec<&str> = s
                .split(|c: char| r.text1.contains(c))
                .filter(|p| !p.is_empty())
                .collect();
            let pat: Vec<char> = r.text2.chars().collect();
            let mut out = String::new();
            let mut i = 0;
            while i < pat.len() {
                if pat[i] == '{' {
                    let mut j = i + 1;
                    let mut digits = String::new();
                    while j < pat.len() && pat[j].is_ascii_digit() {
                        digits.push(pat[j]);
                        j += 1;
                    }
                    if j < pat.len() && pat[j] == '}' && !digits.is_empty() {
                        let n: usize = digits.parse().unwrap_or(0);
                        if n == 0 {
                            out.push_str(s);
                        } else if n <= parts.len() {
                            out.push_str(parts[n - 1]);
                        }
                        i = j + 1;
                        continue;
                    }
                }
                out.push(pat[i]);
                i += 1;
            }
            out
        }
        Kind::Padding => {
            let chars: Vec<char> = s.chars().collect();
            let width = r.n1.clamp(0, 50) as usize;
            let mut out = String::new();
            let mut i = 0;
            while i < chars.len() {
                if chars[i].is_ascii_digit() {
                    let mut j = i;
                    let mut run = String::new();
                    while j < chars.len() && chars[j].is_ascii_digit() {
                        run.push(chars[j]);
                        j += 1;
                    }
                    if r.choice == 0 {
                        while run.len() < width {
                            run.insert(0, '0');
                        }
                    } else {
                        while run.len() > 1 && run.starts_with('0') {
                            run.remove(0);
                        }
                    }
                    out.push_str(&run);
                    i = j;
                } else {
                    out.push(chars[i]);
                    i += 1;
                }
            }
            out
        }
        Kind::Transliterate => {
            let mut out = String::new();
            for c in s.chars() {
                match translit_char(c) {
                    Some(t) => out.push_str(t),
                    None => out.push(c),
                }
            }
            out
        }
        Kind::AssignNames => {
            let lines: Vec<&str> = r.text1.lines().collect();
            match lines.get(index) {
                Some(l) if !l.trim().is_empty() => l.trim().to_string(),
                _ => s.to_string(),
            }
        }
        Kind::MapNames => {
            for line in r.text1.lines() {
                let pair = if let Some(p) = line.find('\t') {
                    Some((&line[..p], &line[p + 1..]))
                } else {
                    line.find('=').map(|p| (&line[..p], &line[p + 1..]))
                };
                if let Some((old, new)) = pair {
                    if old.trim().to_lowercase() == s.to_lowercase() {
                        return new.trim().to_string();
                    }
                }
            }
            s.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_keeps_hidden_files() {
        assert_eq!(split_name("a.b.txt"), ("a.b".to_string(), "txt".to_string()));
        assert_eq!(split_name(".gitignore"), (".gitignore".to_string(), String::new()));
    }

    #[test]
    fn many_rules_are_allowed() {
        let mut rules = Vec::new();
        for _ in 0..12 {
            rules.push(Rule::new(Kind::CleanUp));
        }
        let (s, e) = apply_rules(&rules, " a  b ", "txt", 0, 1);
        assert_eq!(s, "a b");
        assert_eq!(e, "txt");
    }

    #[test]
    fn serialize_counts() {
        let mut r = Rule::new(Kind::Number);
        r.choice = 1;
        r.text1 = "_".to_string();
        r.n3 = 3;
        let (s, _) = apply_rules(&[r], "pic", "jpg", 4, 5);
        assert_eq!(s, "pic_005");
    }

    #[test]
    fn replace_all_ignoring_case() {
        let mut r = Rule::new(Kind::Replace);
        r.text1 = "AB".to_string();
        r.text2 = "x".to_string();
        let (s, _) = apply_rules(&[r], "abcab", "", 0, 1);
        assert_eq!(s, "xcx");
    }

    #[test]
    fn new_extension_rule_keeps_extension() {
        let r = Rule::new(Kind::Extension);
        let (s, x) = apply_rules(&[r], "Photo", "JPG", 0, 1);
        assert_eq!((s.as_str(), x.as_str()), ("Photo", "jpg"));
    }

    #[test]
    fn reserved_names_are_invalid() {
        assert!(invalid_name("CON"));
        assert!(invalid_name("con.txt"));
        assert!(invalid_name("LPT1.log"));
        assert!(!invalid_name("console.txt"));
    }

    #[test]
    fn huge_step_does_not_overflow() {
        let mut r = Rule::new(Kind::Number);
        r.n2 = i64::MAX;
        let _ = apply_rules(&[r], "pic", "jpg", 5, 6);
    }

    #[test]
    fn rearrange_swaps() {
        let r = Rule::new(Kind::Rearrange);
        let (s, _) = apply_rules(&[r], "one-two", "", 0, 1);
        assert_eq!(s, "two one");
    }

    fn numbered(r: &Rule, total: usize) -> Vec<String> {
        (0..total)
            .map(|i| {
                let (s, e) = apply_rules(std::slice::from_ref(r), "foto", "jpg", i, total);
                format!("{}.{}", s, e)
            })
            .collect()
    }

    #[test]
    fn numbering_ascending_with_prefix_and_suffix() {
        let mut r = Rule::new(Kind::Numbering);
        r.text1 = "Viaggio ".to_string();
        r.text2 = " bis".to_string();
        assert_eq!(
            numbered(&r, 3),
            ["Viaggio 01 bis.jpg", "Viaggio 02 bis.jpg", "Viaggio 03 bis.jpg"]
        );
    }

    #[test]
    fn numbering_descending_gives_first_file_the_highest_number() {
        let mut r = Rule::new(Kind::Numbering);
        r.flag1 = true;
        assert_eq!(numbered(&r, 3), ["03.jpg", "02.jpg", "01.jpg"]);
    }

    #[test]
    fn numbering_respects_start_step_and_digits() {
        let mut r = Rule::new(Kind::Numbering);
        r.n1 = 10;
        r.n2 = 5;
        r.n3 = 0;
        assert_eq!(numbered(&r, 3), ["10.jpg", "15.jpg", "20.jpg"]);
        r.flag1 = true;
        assert_eq!(numbered(&r, 3), ["20.jpg", "15.jpg", "10.jpg"]);
    }

    #[test]
    fn numbering_placement_before_and_after_name() {
        let mut r = Rule::new(Kind::Numbering);
        r.text1 = "_".to_string();
        r.choice = 1;
        assert_eq!(numbered(&r, 2), ["foto_01.jpg", "foto_02.jpg"]);
        r.text1 = String::new();
        r.text2 = "_".to_string();
        r.choice = 0;
        assert_eq!(numbered(&r, 2), ["01_foto.jpg", "02_foto.jpg"]);
    }

    #[test]
    fn numbering_unmarked_file_past_the_end_does_not_panic() {
        let mut r = Rule::new(Kind::Numbering);
        r.flag1 = true;
        let _ = apply_rules(&[r], "foto", "jpg", 7, 3);
    }
}
