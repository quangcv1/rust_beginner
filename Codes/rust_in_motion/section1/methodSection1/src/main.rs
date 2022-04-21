fn main() {
    println!("Hello, world!");
    let mut player = HockeyPlayer{
        name: String::from("Quang"),
        number: 17,
        position: HockeyPosition::Wing,
        goals_ytd: 7,
    };
    player.goals_ytd += 1;
    //shoot_puck(player,1000);
    player.shoot_puck(200);

    let mut player1 = HockeyPlayer::new(
        String::from("Minh"),
        6,
        HockeyPosition::Wing,
    );
    player1.shoot_puck(300);
    player1.shoot_puck(200);
}

impl HockeyPlayer {
    // fn new(name: String, number:u8, position: HockeyPosition) -> HockeyPlayer {
    //     HockeyPlayer{
    //         name,
    //         number,
    //         position,
    //         goals_ytd: 0,
    //     }
    // }
    fn shoot_puck(&mut self, seconds_remaining: u16) {
        if seconds_remaining < 300 {
            match self.position {
                HockeyPosition::Center => {
                    self.goals_ytd += 1;
                    println!("Goal!");
                },
                _ => println!("Miss!"),
            }
        } else {
            self.goals_ytd +=1;
            println!("Goal!");
        }
    }
}

impl HockeyPlayer {
    fn new(name: String, number:u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer{
            name,
            number,
            position,
            goals_ytd: 0,
        }
    }
}


fn shoot_puck(hockey_player: HockeyPlayer, seconds_remaining: u16) {
    if seconds_remaining < 300 {
        match hockey_player.position {
            HockeyPosition::Center => println!("Goal"),
            _ => println!("Miss"),
        }
    } else {
        println!("Goal!");
    }
}
enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}