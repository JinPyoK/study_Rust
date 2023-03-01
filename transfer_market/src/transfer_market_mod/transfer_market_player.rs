pub struct SoccerPlayer
{
    name: String,
    price: i64, //transfer fee
}

impl SoccerPlayer
{
    pub fn new(name: String, price: i64) -> Self {
        SoccerPlayer {name, price}
    }

    pub fn print_player(&self) {
        println!("Name: {}, Price: {}", self.name, self.price);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_price(&self) -> i64 {
        self.price
    }
}

pub struct PlayerBucket
{
    player: SoccerPlayer,
    count: i64, // num of players
}

impl PlayerBucket
{
    pub fn new(player: SoccerPlayer, count: i64) -> Self {
        PlayerBucket {player, count}
    }

    pub fn print_bucket(&self) {
        self.player.print_player();
        println!("Count: {}", self.count);
    }
    pub fn get_player_to_string(&self) -> String {
        format!("Name: {}, Price: {}, Count: {}", self.player.get_name(), self.player.get_price(), self.get_count())
    }
    pub fn get_player(&self) -> &SoccerPlayer {
        &self.player
    }
    pub fn get_count(&self) -> i64 {
        self.count
    }
    pub fn count_change(&mut self, val: i64) {
        self.count += val;
    }
}