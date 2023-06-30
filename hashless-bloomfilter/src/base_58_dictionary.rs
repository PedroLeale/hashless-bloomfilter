pub fn from_base58_dictionary(value: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for c in value.chars() {
        match char_to_int(c) {
            Some(n) => result.push(n),
            None => (),
        }
    }
    result
}


#[inline(always)]
fn char_to_int(c: char) -> Option<u8> {
    match c {
        '1' => Some(0),
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        '9' => Some(8),
        'A' => Some(9),
        'B' => Some(10),
        'C' => Some(11),
        'D' => Some(12),
        'E' => Some(13),
        'F' => Some(14),
        'G' => Some(15),
        'H' => Some(16),
        'J' => Some(17),
        'K' => Some(18),
        'L' => Some(19),
        'M' => Some(20),
        'N' => Some(21),
        'P' => Some(22),
        'Q' => Some(23),
        'R' => Some(24),
        'S' => Some(25),
        'T' => Some(26),
        'U' => Some(27),
        'V' => Some(28),
        'W' => Some(29),
        'X' => Some(30),
        'Y' => Some(31),
        'Z' => Some(32),
        'a' => Some(33),
        'b' => Some(34),
        'c' => Some(35),
        'd' => Some(36),
        'e' => Some(37),
        'f' => Some(38),
        'g' => Some(39),
        'h' => Some(40),
        'i' => Some(41),
        'j' => Some(42),
        'k' => Some(43),
        'm' => Some(44),
        'n' => Some(45),
        'o' => Some(46),
        'p' => Some(47),
        'q' => Some(48),
        'r' => Some(49),
        's' => Some(50),
        't' => Some(51),
        'u' => Some(52),
        'v' => Some(53),
        'w' => Some(54),
        'x' => Some(55),
        'y' => Some(56),
        'z' => Some(57),
        _ => None,
    }
}
