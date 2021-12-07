from hydrothermal_venture.vent import Vent


class Vents:
    def read(file_path):
        vents = []
        with open(file_path) as file:
            for line in file.readlines():
                points = line.split(" -> ")
                start = points[0].split(",")
                end = points[1].split(",")
                vents.append(Vent(int(start[0]), int(start[1]), int(end[0]), int(end[1])))
        return vents