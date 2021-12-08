import sys
import multiprocessing as mp

def main():
    file_path = sys.argv[2]
    lanternfishes = []
    with open(file_path) as file:
        for number in file.read().split(","):
            lanternfishes.append(int(number))

    pool = mp.Pool(mp.cpu_count())
    results = [pool.apply(simulate, args=[lanternfishes])]
    
    numberOfLanternfishes = 0
    for result in results:
        numberOfLanternfishes += result

    print(numberOfLanternfishes)

def simulate(lanternfishes):
    for _ in range(int(sys.argv[1])):
        new_generation = []
        for lanternfish in lanternfishes:
            if (lanternfish == 0):
                new_generation.append(6)
                new_generation.append(8)
            else:
                new_generation.append(lanternfish-1)
        lanternfishes = new_generation
    return len(lanternfishes)


if __name__ == "__main__":
    main()