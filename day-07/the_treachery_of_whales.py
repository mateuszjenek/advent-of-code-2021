import sys
import numpy as np


def main():
    file_path = sys.argv[1]
    crab_positions = []
    with open(file_path) as file:
        for number in file.read().split(","):
            crab_positions.append(int(number))
    
    fuel_consumptions = []
    for position in range(min(crab_positions), max(crab_positions)):
        fuel_consumption = 0
        for crab in crab_positions:
            distance = abs(position-crab)
            fuel_consumption += (distance*(distance+1))/2
        fuel_consumptions.append(int(fuel_consumption))

    print(min(fuel_consumptions))

if __name__ == "__main__":
    main()