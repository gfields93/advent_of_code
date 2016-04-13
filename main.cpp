////////////////////////////
//Glen Fields main.cpp
////////////////////////////

#include "boost/lexical_cast.hpp"
#include "ctype.h"
#include "Functions.h"
#include <vector>

int main(int argc, const char * argv[]){
	if (argc != 6) {
		printf("This program requires 5 inputs to be given when it is run.\n"
			"1. The total simulation time.\n"
			"2. The time it takes for the plane to land.\n"
			"3. The time it takes for the plane to takeoff.\n"
			"4. The amount of planes that land in a hour.\n"
			"5. The amount of planes that takeoff in a hour.\n"
			"All of these should be given as integers.");
		return -1;
	}

	std::vector<int> args(5);
	for(int i = 0; i < argc-1; i++){
		try{
			args[i] = boost::lexical_cast<int>(argv[i+1]);
		}
		catch(const boost::bad_lexical_cast &){
			std::cout << "Argument " << i <<" is not a valid value." << std::endl;
			return -1;
		}
	}

	displayArray(airportSim(args[1], args[2], args[3],  args[4], args[5]));
	return 0;
}