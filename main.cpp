////////////////////////////
//Glen Fields main.cpp
////////////////////////////


#include "Functions.h"

int main(){
	//testing airportSim() with five test
	displayArray(airportSim(1440, 3, 4, 10, 15));
	std::cout << std::endl;
	displayArray(airportSim(300, 10, 2, 6, 10));
	std::cout << std::endl;
	displayArray(airportSim(60, 0, 10, 0, 10));
	std::cout << std::endl;
	displayArray(airportSim(60, 10, 0, 10, 0));
	std::cout << std::endl;
	displayArray(airportSim(300, 5, 8, 20, 15));
	return 0;
}