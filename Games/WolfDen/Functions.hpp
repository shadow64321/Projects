#include <string>
#include "SaveFiles/Save.wdsf"

bool Walkies = false;
bool Food = false;
bool Sleep = false;
bool Explore = false;


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
    WeatherCycleStepVal + 0.01;
    cout << WeatherCycleStepVal;
}

int WildlifeMigrationCycle() {
    WildlifeMigrationStepVal + 0.01;
    cout << WildlifeMigrationStepVal;
}

int seasonCycle() {
    seasonCycleStep + 1;
    if (seasonCycleStep = 1000) {
        Season = "Summer";
        cout << Season;
    } else if (seasonCycleStep = 2000) {
        Season = "Autumn";
        cout << Season;
    } else if (seasonCycleStep = 3000) {
        Season = "Winter";
        cout << Season;
    } else if (seasonCycleStep = 4000) {
        Season = "Spring";
        cout << Season;
    }
}

int MainMenu() {
    MainMenuReqLoad = true;

}