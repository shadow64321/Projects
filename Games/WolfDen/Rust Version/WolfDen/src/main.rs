use std::io::stdin;

fn main() {
    let mut MainMenuReqLoad = false;
    let mut WeatherCycleStepVal = 0.0;
    let mut WildlifeMigrationCycleStepVal = 0.0;
    let mut Season = String::new();
    let mut SeasonCycleStep = 0;

    println!("Welcome to WolfDen");
    MainMenuReqLoad = true;
    if MainMenuReqLoad == true {
        MainMenuLoad();
        MainMenuReqLoad = false;
    }
}

fn MainMenuLoad() {
    println!("WolfDen\n1|Start Game\n2|Caracter Customisation\n");

}