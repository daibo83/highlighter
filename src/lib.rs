use memchr::memmem::Finder;
///lowercase and remove diacritics
pub fn preprocess_input(input: &str) -> String {
    let input = input
        .trim()
        .chars()
        .map(|x| match x {
            'À' | 'Á' | 'Ả' | 'Ạ' | 'Ã' | 'Â' | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'Ä' | 'Å' | 'Æ'
            | 'Ă' | 'Ắ' | 'Ằ' | 'Ẵ' | 'Ẳ' | 'Ặ' => 'a',
            'Þ' => 'b',
            'Ç' | 'Č' => 'c',
            'Ď' | 'Ð' => 'd',
            'Ě' | 'È' | 'É' | 'Ẽ' | 'Ẻ' | 'Ẹ' | 'Ê' | 'Ế' | 'Ề' | 'Ễ' | 'Ể' | 'Ệ' | 'Ë' => {
                'e'
            }
            'Ƒ' => 'f',
            'Ì' | 'Í' | 'Ĩ' | 'Ỉ' | 'Ị' | 'Î' | 'Ï' => 'i',
            'Ň' | 'Ñ' => 'n',
            'Ò' | 'Ó' | 'Õ' | 'Ỏ' | 'Ọ' | 'Ô' | 'Ố' | 'Ồ' | 'Ỗ' | 'Ổ' | 'Ộ' | 'Ơ' | 'Ớ' |'Ờ'|'Ỡ'|'Ở'|'Ợ' |'Ö' | 'Ø' => {
                'o'
            }
            'Ř' => 'r',
            'Š' => 's',
            'Ť' => 't',
            'Ů' | 'Ù' | 'Ú' | 'Ũ' | 'Ủ' | 'Ụ' |'Ư'|'Ứ'|'Ừ'|'Ữ'|'Ử'|'Ự'|'Û' | 'Ü' => 'u',
            'Ý'|'Ỳ'|'Ỹ'|'Ỷ'|'Ỵ' => 'y',
            'Ž' => 'z',

            'à' | 'á' | 'ã' | 'ả' | 'ạ' | 'â' | 'ấ' | 'ầ' | 'ẫ' | 'ẩ' | 'ậ' | 'ă' | 'ắ' | 'ằ'
            | 'ẵ' | 'ẳ' | 'ặ' | 'ä' | 'å' | 'æ' => 'a',
            'þ' => 'b',
            'ç' | 'č' => 'c',
            'ď' | 'ð' |'đ'=> 'd',
            'ě' | 'è' | 'é' | 'ẽ' | 'ẻ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ễ' | 'ể' | 'ệ' | 'ë' => 'e',
            'ƒ' => 'f',
            'ì' | 'í' | 'ĩ' | 'ỉ' | 'ị' | 'î' | 'ï' => 'i',
            'ñ' | 'ň' => 'n',
            'ò' | 'ó' | 'õ' | 'ỏ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ỗ' | 'ổ' | 'ộ' |'ơ'|'ớ'|'ờ'|'ỡ'|'ở'|'ợ'| 'ö' | 'ø' => 'o',
            'ř' => 'r',
            'š' => 's',
            'ť' => 't',
            'ů' | 'ù' | 'ú' |'ũ'|'ủ'|'ụ'|'ư'|'ứ'|'ừ'|'ữ'|'ử'|'ự'| 'û' | 'ü' => 'u',
            'ý' |'ỳ'|'ỹ'|'ỷ'|'ỵ'| 'ÿ' => 'y',
            'ž' => 'z',
            'A'..='Z' => x.to_ascii_lowercase(),
            'a'..='z' => x,
            '0'..='9' => x,
            _ => ' ',
        })
        // .filter(|c| c.is_ascii())
        .collect::<String>();
    // input = input
    //     .chars()
    //     .map(|x| match x {
    //         // 'A'..='Z' => x,
    //         'a'..='z' => x,
    //         '0'..='9' => x,
    //         _ => ' ',
    //     })
    //     .collect();
    // input = input.trim().to_string();
    // input = input.replace('  ', ' ').replace('  ', ' ');
    // println!("{}", input);
    input
}
pub trait StartsWith<T> {
    fn starts_with(&self, needle: &[T]) -> bool;
}

impl StartsWith<u8> for [u8] {
    #[inline(always)]
    fn starts_with(&self, needle: &[u8]) -> bool {
        if needle.len() > self.len() {
            false
        }
        else {
            &self[..needle.len()] == needle
        }
    }
}
#[inline(always)]
pub fn highlight(needles: &[&str], haystack: &str, opening_tag: &str, closing_tag: &str) -> String {
    let mut needles_dedup = needles.iter().map(|needle| preprocess_input(needle)).collect::<Vec<String>>();
    needles_dedup.dedup();
    // needles_dedup.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    let mut haystacks: Vec<String> = haystack.split_whitespace().map(|s|s.to_owned()).collect();
    let haystacks_processed: Vec<String> = haystacks.iter().map(|s|preprocess_input(s)).collect();
    for (i, haystack) in haystacks_processed.iter().enumerate() {
        for needle in needles_dedup.iter() {
            if haystack.as_bytes().starts_with(needle.as_bytes()) {
                let mut tagged = opening_tag.to_string();
                tagged.reserve(haystack.len() + closing_tag.len());
                tagged.push_str(unsafe{haystacks.get_unchecked(i)});
                tagged.push_str(&closing_tag.to_string());
                unsafe{*haystacks.get_unchecked_mut(i) = tagged;}
                break;
            }
        }
    }
    let res: String = haystacks.join(" ");
    res
}

#[inline(always)]
pub fn get_highlight_offsets(needles: &[&str], haystack: &str) -> Vec<(usize, usize)> {
    let mut needles_dedup = needles.to_owned();
    needles_dedup.dedup();
    let mut matches: Vec<(usize, usize)> = Vec::with_capacity(10);
    let haystack_lc = haystack.to_lowercase();
    for needle in needles_dedup {
        let searcher = Finder::new(needle);
        let mut matches_to_add = Vec::new();
        let new_matches: Vec<(usize, usize)> = searcher.find_iter(haystack_lc.as_bytes()).map(|x| (x, x + needle.len())).collect();
        if !matches.is_empty() {
            for new_match in new_matches {
                for added_match in &matches {
                    if !((added_match.0..added_match.1).contains(&new_match.0)) {
                        if added_match == matches.last().unwrap() {
                            matches_to_add.push((new_match.0, new_match.1));
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            matches.extend(new_matches);
        }
        matches.append(&mut matches_to_add);
    }
    matches
}

#[inline(always)]
pub fn get_highlight_offsets_new(needles: &[&str], haystack: &str) -> Vec<(usize, usize)> {
    let mut needles_dedup = needles.to_owned();
    needles_dedup.dedup();
    needles_dedup.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    let mut matches: Vec<(usize, usize)> = Vec::with_capacity(10);
    let mut start = 0;
    for haystack_part in haystack.split_whitespace() {
        let haystack_part_lc = haystack_part.to_lowercase();
        for needle in needles_dedup.iter() {
            if haystack_part_lc.as_bytes().starts_with(needle.as_bytes()) {
                
                matches.push((start, start + haystack_part_lc.len()));
                break;
            }
        }
        start += haystack_part.len() + 1;
    }
    matches
}