////////////////////////////
//Glen Fields main.cpp
////////////////////////////


#include "Functions.h"

int main(){
	//testing by prime factors by picking 10 random numbers
	int n = 10;
	srand(time(NULL));
	for (size_t i = 0; i < n; i++)
	{
		displayStack(primeFactors((rand() % 9999999) + 2));
	}

	//testing airportSim() with five test
	std::cout << std::endl;
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