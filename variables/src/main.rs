fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    const MISSION_COMPLETE: &str = "Mission Complete, let's go home"; 
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left...", missiles);

    let (remaining_missiles, mut target)= (missiles, "target missed");

    println!("{} you have {} more shots", target, remaining_missiles);
    
    target="target acquired";
    println!("{}",target);
    println!("Firing all {} missiles!", remaining_missiles);
    target = "DIRECT HIT!!!";
    println!("{}",target);
    println!("{}",MISSION_COMPLETE);
    

}
