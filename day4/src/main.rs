use md5;

const start: &[u8; 8] = b"bgvyzdsv";
const zeroes: &str = "000000";

fn main() {
    let mut st: Vec<u8> = Vec::with_capacity(16);
    for byte in start.iter() {
        st.push(*byte);
    }

    let mut i = 0;
    loop {
        i += 1;
        let mut str = st.clone();
        let str_i = i.to_string();
        str.extend(str_i.into_bytes());

        let digest = md5::compute(str);
        let digest_str = format!("{:x}", digest);
        if &digest_str[0..6] == zeroes {
            println!("Ding Ding Ding! {} -> {}", i, digest_str);
            break;
        }
    }
}
