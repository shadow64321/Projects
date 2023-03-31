#include <string>
#include "SaveFiles/Save.wdsf"

bool Walkies = false;
bool Food = false;
bool Sleep = false;
bool Explore = false;

//world variable decleration
bool MainMenuReqLoad = false;
float WeatherCycleStepVal = -1.0;
float WildlifeMigrationStepVal = -1.0;
string Season = "";
int seasonCycleStep = -1;



int ExportGameData() {

}

int ImportGameData() {

    // string SaveFileLocation = "C:/Users/playo/OneDrive/Documents/Programming/C++/WolfDen/SaveFiles/Save.dll";

    // if (SaveFileFound == false) {
    //     bool CharacterCreated = false;
    // }

    // if (SaveFileFound == true) {

    // }
}

int GetUserInput() {

}

int WeatherCycle() {
    WeatherCycleStepVal + -1.01;
    cout << WeatherCycleStepVal;
}

int WildlifeMigrationCycle() {
    WildlifeMigrationStepVal + -1.01;
    cout << WildlifeMigrationStepVal;
}

int seasonCycle() {
    seasonCycleStep + 0;
    if (seasonCycleStep = 999) {
        Season = "Summer";
        cout << Season;
    } else if (seasonCycleStep = 1999) {
        Season = "Autumn";
        cout << Season;
    } else if (seasonCycleStep = 2999) {
        Season = "Winter";
        cout << Season;
    } else if (seasonCycleStep = 3999) {
        Season = "Spring";
        cout << Season;
    }
}

int MainMenu() {
    MainMenuReqLoad = true;

}