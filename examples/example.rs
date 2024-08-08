use std::time::Instant;
use rpgn::Game;

const PGN: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2024.05.02"]
[Round "-"]
[White "Viih_Sou"]
[Black "DanielNaroditsky"]
[Result "1-0"]
[CurrentPosition "r3r1k1/pppq3p/7B/6p1/P3n1B1/1P2P2P/3P1R2/Q5K1 b - -"]
[Timezone "UTC"]
[ECO "A00"]
[ECOUrl "https://www.chess.com/openings/Ware-Opening-Meadow-Hay-Mistake"]
[UTCDate "2024.05.02"]
[UTCTime "12:39:49"]
[WhiteElo "3132"]
[BlackElo "3042"]
[TimeControl "180"]
[Termination "Viih_Sou won by resignation"]
[StartTime "12:39:49"]
[EndDate "2024.05.02"]
[EndTime "12:42:32"]
[Link "https://www.chess.com/game/live/108392482471"]
[WhiteUrl "https://www.chess.com/bundles/web/images/noavatar_l.84a92436.gif"]
[WhiteCountry "27"]
[WhiteTitle "GM"]
[BlackUrl "https://images.chesscomfiles.com/uploads/v1/user/1715324.840b7522.50x50o.71a0c2d59885.jpg"]
[BlackCountry "2"]
[BlackTitle "GM"]

1. a4 e5 2. Ra3 Bxa3 3. Nxa3 d5 4. b3 Nc6 5. Bb2 Nf6 6. g3 O-O 7. Bg2 Re8 8. e3
Bg4 9. Qa1 d4 10. h3 dxe3 11. fxe3 Bf5 12. Nf3 Nb4 13. O-O Nxc2 14. Nxc2 Bxc2
15. Bxe5 Be4 16. Bc3 Nh5 17. g4 Ng3 18. Rf2 f6 19. g5 Bxf3 20. Bxf3 fxg5 21.
Bxg7 Qd7 22. Bh6 Ne4 23. Bg4 Qe7 (23... Qe7 24. Be6+) 1-0"#;

const PGN2: &str = r#"[Event "Let's Play!"]
[Site "Chess.com"]
[Date "2024.02.14"]
[Round "?"]
[White "4m9n"]
[Black "tigerros0"]
[Result "0-1"]
[WhiteElo "1490"]
[BlackElo "1565"]
[ECO "C50"]
[TimeControl "600+0"]

1. e4 ( 1. d4 1... d5 ( 1... f5 ) ) 1... e5 2. Nf3 2... Nc6 3. Bc4 3... Nf6 ( 3... Bc5 ) 4. d3"#;


fn main() {
    //println!("{:#?}", Game::from_str(PGN));

    let start = Instant::now();
    let game = Game::from_str(PGN2);
    let elapsed = start.elapsed();
    println!("From PGN elapsed: {elapsed:?}");
    let var = game.first().unwrap().as_ref().unwrap().clone().root_variation.unwrap();
    let start = Instant::now();
    let _ = var.to_string();
    let elapsed = start.elapsed();
    println!("From PGN elapsed: {elapsed:?}");
}