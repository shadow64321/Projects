//raccoon
//Duckbot
//character creation -- in progress
//main menu -In Prograss
//weather cycles
//wildlife migraton cycle
//location select/fast travel: jungle, forest, rainforest
//Combat system and random enemy encounter system


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