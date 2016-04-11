///////////////////////////////
//Glen Fields Function.h
///////////////////////////////

#ifndef FUNCTIONS_H_
#define FUNCTIONS_H_

#include <stdbool.h>
#include <stdlib.h>
#include <stack>
#include <queue>
#include <array>
#include <time.h>
#include <iostream>

/**
* Determine if a new plane is ready for either takeoff
* or landing according to the rate passed in. If this function
* is called 60 times, the number of returned trues is approximately
* equal to the original rate passed in.
*
* Precondition: A rate defining the average number of
* departures/arrivals per hour.
* Postcondition: A bool indicating if there is a plane
* ready to land/takeoff (dependent upon the rate passed in)
*
* Time Complexity: Theta(1)
*/
bool isReady(int rate)
{
	// rate/60 gives us the likelihood that the indicated
	//action will happen.
	rate = (int)(((float)rate / 60.0) * 100);

	if (rand() % 100 <= rate) return true;
	else                   return false;
}

std::stack<int> primeFactors(int n);
std::array<int, 4> airportSim(int totalSimTime, int landingTime, int takeoffTime,
	int landingRate, int takeoffRate);
void displayStack(std::stack<int> stack); //displays result of primeFactors()
void displayArray(std::array<int, 4> array); //displays result of airportSim()
///////////////////////////////////////////////////////////////////////

std::stack<int> primeFactors(int n){
	int divisor = 2;	//starting divisor
	if (n <= 1){		//checks if value is invalid
		std::stack<int> factors;
		factors.push(1);
		return factors;
	}
	else{				//fills stack with factors
		std::stack<int> factors;
		while (n > 1){
			if (n % divisor == 0)
			{
				factors.push(divisor);
				n /= divisor;
				divisor = 2;
			}
			else{
				++divisor;
			}
		}
		std::stack<int> stack;
		while (!factors.empty()) //puts stack in ascending order
		{
			stack.push(factors.top());
			factors.pop();
		}
		return stack;
	}
}

std::array<int, 4> airportSim(int totalSimTime, int landingTime, int takeoffTime,
	//needed variables
	int landingRate, int takeoffRate){
	srand(time(NULL));
	std::array<int, 4> array;
	std::queue<int> takeoffQueue, landingQueue;
	int takeoffLength = 0, totalTakeoff = 0;
	int landingLength = 0, totalLanding = 0;
	int lT = 0;
	int tT = 0;
	int totalLandingTime = 0, totalTakeoffTime = 0;
	int averageLandingLength = 0, averageTakeoffLength = 0;

	//for begins simulation
	for (int clock = 0; clock < totalSimTime; ++clock){
		if (isReady(takeoffRate)) //takeoff check
		{
			takeoffQueue.push(clock);
			++takeoffLength;
		}

		if (isReady(landingRate)) //landing check
		{
			landingQueue.push(clock);
			++landingLength;
		}

		averageLandingLength += landingQueue.size();
		averageTakeoffLength += takeoffQueue.size();

		//checks if plane can land
		if (!landingQueue.empty() && lT == 0 && tT == 0){
			totalLandingTime += (clock - landingQueue.front());
			landingQueue.pop();
			lT = landingTime;
			continue;
		}
		//checks if plane can takeoff 
		else if (!takeoffQueue.empty() && landingQueue.empty() && lT == 0 && tT == 0){
			totalTakeoffTime += (clock - takeoffQueue.front());
			takeoffQueue.pop();
			tT = takeoffTime;
			continue;
		}
		//next two else-if tell if runway is busy
		else if (lT != 0){
			--lT;
			continue;
		}
		else if (tT != 0){
			--tT;
			continue;
		}

	}
	//next find plane information after sim has ended
	while (!landingQueue.empty())
	{
		totalLandingTime += (totalSimTime - landingQueue.front());
		landingQueue.pop();
	}
	while (!takeoffQueue.empty())
	{
		totalTakeoffTime += (totalSimTime - takeoffQueue.front());
		takeoffQueue.pop();
	}

	//if no planes have arrived, prevents division by zero
	if (landingLength == 0)
	{
		landingLength = 1;
	}
	if (takeoffLength == 0){
		takeoffLength = 1;
	}

	//fills array with proper values
	array[0] = (averageLandingLength / totalSimTime) + 1;
	array[1] = (totalLandingTime / landingLength) + 1;
	array[2] = (averageTakeoffLength / totalSimTime) + 1;
	array[3] = (totalTakeoffTime / takeoffLength) + 1;

	return array;
}


//functions after this are helper functions
void displayStack(std::stack<int> stack){
	if (stack.empty())
	{
		std::cout << "The stack is empty.\n";
	}
	else{
		while (!stack.empty())
		{
			std::cout << stack.top() << " ";
			stack.pop();

		}
	}
	std::cout << std::endl;
}

void displayArray(std::array<int, 4> array){
	std::cout << "Average arriving queue length: " << array[0] << " planes\n";
	std::cout << "Average arriving time spent in queue: " << array[1] << " minutes\n";
	std::cout << "Average leaving queue length: " << array[2] << " planes\n";
	std::cout << "Average leaving time spent in queue: " << array[3] << " minutes" << std::endl;
}

#endif