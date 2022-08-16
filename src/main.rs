fn nmea_checksum(message: &str) -> u8 {
    let mut checksum: u8 = 0;
    for (i, c) in message.chars().enumerate() {
        if !(i == 0 && c == '$') {
            checksum ^= c as u8;
        }
    }
    return checksum;
}

fn main() {
    let message: &str = "$CS";
    let checksum = nmea_checksum(message);
    println!("{}*{:x}", message, checksum);
}

#[cfg(test)]
mod tests {
    use crate::nmea_checksum;

    #[test]
    fn ignores_first_dollar_sign() {
        assert_eq!(nmea_checksum("CS"), nmea_checksum("$CS"));
    }

    #[test]
    fn null_checksum() {
        assert_eq!(nmea_checksum("AA"), 0);
        assert_eq!(nmea_checksum("AA"), nmea_checksum("AAAA"));
    }

    #[test]
    fn simple_checksum() {
        assert_eq!(format!("{:x}", nmea_checksum("$CS")), "10");
    }

    #[test]
    fn complex_checksum() {
        assert_eq!(format!("{:x}", nmea_checksum("$PFEC,GPint,RMC05")), "2d");
    }
}