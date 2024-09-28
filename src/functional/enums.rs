pub enum SportPlayer {
    Football,
    Basketball
}

pub fn get_salary(player: SportPlayer) -> i32 {
    let salary = match player {
        SportPlayer::Football => 10000,
        SportPlayer::Basketball => 5000,
    };
    salary
}
