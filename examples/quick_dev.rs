use std::str::FromStr;

use zk_sudoku_prover::*;

fn main() {
    let input = r#"281953647476218593935467128364721985712895436859346271543672819198534762627189354
127935864648127593935648712371459628286371945459286137894762351762513489513894276
296541378851273694743698251915764832387152946624839517139486725478325169562917483
475812369921653847368947215287569134154738926693421758736285491519374682842196573
763184259148259763592763148914572386625348971387691425879436512431825697256917834
872465913564193728391278645745316289613829457928547136257634891436981572189752364
937426185148759623625138749394682571271945368856371294763814952412597836589263417
926453187187926453453187926342715698698342715715698342574861239861239574239574861
932614857148725963765893421813972645496358712257146389321489576589267134674531298
497631582821459736356827194749168253612573849583942671174386925265794318938215467
"#;
    println!("Input: {}", input);
    let line = input.lines().next().unwrap();
    let board = SudokuGrid::from_str(line).unwrap();
    println!("Board:\n{}", board);
    println!("Valid: {}", board.is_valid_solution());

    let mut zk_protocol = ZKProtocol::new(&board).unwrap();

    let t1 = std::time::Instant::now();

    let output = zk_protocol.prove_with_confidence(99.0).unwrap();

    let time_taken = t1.elapsed().as_millis();

    println!("Time taken: {}ms", time_taken);

    println!("Proof: {}", output);
}
