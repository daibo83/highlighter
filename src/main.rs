use highlighter::{get_highlight_offsets, highlight, get_highlight_offsets_new};
use std::time::Instant;
pub fn main() {
    let needle = ["91", "trung", "kinh"];
    let haystack = "phuong trung hoa quan cau giay thanh pho ha noi";
    let now = Instant::now();
    let res = highlight(&needle, haystack, "<marked>", "</marked>");
    println!("{:?}, {:?}", res, now.elapsed());
    let now = Instant::now();
    let res = get_highlight_offsets_new(&needle, haystack);
    println!("{:?}, {:?}", res, now.elapsed());
    let now = Instant::now();
    let res = get_highlight_offsets(&needle, haystack);
    println!("{:?}, {:?}", res, now.elapsed());

    println!("{:?}", "asd, sad".split_whitespace().collect::<Vec<&str>>());
}