use memchr::memmem::Finder;

pub trait StartsWith<T> {
    fn starts_with(&self, needle: &[T]) -> bool;
}

impl StartsWith<u8> for [u8] {
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
    let mut needles_dedup = needles.to_owned();
    needles_dedup.dedup();
    needles_dedup.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    let mut haystacks: Vec<String> = haystack.split_whitespace().map(|s| s.to_lowercase()).collect();
    for haystack in haystacks.iter_mut() {
        for needle in needles_dedup.iter() {
            if haystack.as_bytes().starts_with(needle.as_bytes()) {
                let mut tagged = opening_tag.to_string();
                tagged.reserve(haystack.len() + closing_tag.len());
                tagged.push_str(haystack);
                tagged.push_str(&closing_tag.to_string());
                *haystack = tagged;
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