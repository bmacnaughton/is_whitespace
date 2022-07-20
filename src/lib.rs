pub mod map_lookup;

#[cfg(test)]
mod tests {
    #[test]
    fn standard_library_matches_new_version() {
        for c in 0..=0x4002 {
            let ch = char::from_u32(c).unwrap();
            let l = ch.is_whitespace();
            let u = crate::map_lookup::white_space::lookup(ch);
            assert!(l == u, "is_whitespace must match new implementation");
        }
    }
}
