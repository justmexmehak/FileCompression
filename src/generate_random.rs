use rand::Rng;

pub fn generate_random_data() -> String {
    let mut data = String::new();

    for _ in 1..=10000 {
        let ch: u8 = match rand::thread_rng().gen_range(0..=1) {
            0 => rand::thread_rng().gen_range(65..=90),
            1 => rand::thread_rng().gen_range(97..=122),
            _ => todo!(),
        };
        let ch = ch as char;
        for _ in 1..=rand::thread_rng().gen_range(1..=100) {
            data.push(ch);
        }
    }
    data
}