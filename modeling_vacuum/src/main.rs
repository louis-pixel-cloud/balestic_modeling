mod physic;
use std :: io;

fn main() {
    let mut guess : String = String :: new ();
    let mut weapon_number: i32 = 0;
    loop{
        println!("Choose your weapon : ");
        println!("M4 (1) / DSRT_EAGLES (2)");
        io :: stdin().read_line(&mut guess).expect("there is an error");
        weapon_number = match guess.trim().parse::<i32>(){
            Ok(num )=> num,
            Err(_)=> continue,
        };
        if weapon_number != 0{
            break;
        }
    }
    println!("weapon number : {}" , weapon_number);
    let mut angle : i32 = 0;
    loop{
        println!("Choose your angles : (between 0 and 90 degrees) :");
        guess = String :: new();
        io :: stdin().read_line(&mut guess).expect("there is an error");
        println!("guess : {}", guess);
        angle = match guess.trim().parse::<i32>(){
            Ok(num )=> num,
            Err(_)=> continue,
        };
        if angle != 0{
            break;
        }
    }
    let firearm : weapon :: Weapon;
    if weapon_number == 1 {
        firearm = weapon :: M4 ;
    }
    else if weapon_number == 2 {
        firearm = weapon ::DSRT_EAGLES;
    }
    else{
        println!("there is an error :(");
        return;
    }
    println!("initial speed : {}, chargor capacity : {} , angles value : {} " , firearm.initial_speed , firearm.chargor_capacity , angle);
    println!("scope : {}  , top : ({} , {})" , physic :: getScope(angle , firearm) , (physic :: getTop(angle)).x , (physic :: getTop(angle)).y);

}
