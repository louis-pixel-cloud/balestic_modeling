#[allow(dead_code)]
pub struct Weapon {
    pub initial_speed : i32 , // in m/s
    pub chargor_capacity : i32 ,
}

#[allow(dead_code)]
pub const M4: Weapon = Weapon{
    initial_speed : 880,
    chargor_capacity : 50,
};

#[allow(dead_code)]
pub const DSRT_EAGLES : Weapon = Weapon{
    initial_speed : 470,
    chargor_capacity : 9,
};