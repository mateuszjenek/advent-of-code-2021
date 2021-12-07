import sys
from hydrothermal_venture.hydrothermal_map import HydrothremalMap
from hydrothermal_venture.vents import Vents


def main():
    hydrothermal_map = HydrothremalMap.empty(1000, 1000)
    vents = Vents.read(sys.argv[1])
    for vent in vents:
        hydrothermal_map.apply(vent)
    
    number = hydrothermal_map.filter(lambda cell: cell > 1).counts()
    print(number)

if __name__ == "__main__":
    main()