#include <string>
#include "SaveFiles/Save.wdsf"
#include <iostream>
#include <ctime>
#include "Functions.hpp"
#include "SaveFiles/Save.wdsf"

using namespace std;

//world function decleration
void pause(int dur);


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

#include <iostream>
#include <ctime>
#include "Functions.hpp"
#include "SaveFiles/Save.wdsf"

using namespace std;

//world function decleration
void pause(int dur);


int main() {
    ImportGameData();
    MainMenu();

    const bool MainMenuReqLoad = false;
    bool DoingStuff = false;

    int input = 0;

    if (DoingStuff == false) {
        pause(5);
        cout << "what do you want to do?";
        cin >> input;
        switch (input) {
            case 1:
                DoingStuff = true;
                Walkies = true;
                cout << "Walking";
                pause(5);
                cout << "..........................................\n";
                Walkies = false;
                DoingStuff = false;
                break;

            case 2:
                DoingStuff = true;
                Food = true;
                cout << "Eating";
                pause(5);
                cout << "..........................................\n";
                Food = false;
                DoingStuff = false;
                break;

            case 3:
                DoingStuff = true;
                Sleep = true;
                cout << "Sleeping";
                pause(5);
                cout << "..........................................\n";
                Sleep = false;
                DoingStuff = false;
                break;

            case 4:
                DoingStuff = true;
                Explore = true;
                cout << "Exploring";
                pause(5);
                cout << "..........................................\n";
                Explore = false;
                DoingStuff = false;
                break;
        }


        WildlifeMigrationCycle();
        WeatherCycle();
        seasonCycle();
        // end of game loop cycle
    }
}