from hydrothermal_venture.vent import Vent
import numpy as np

class HydrothremalMap:
    def __init__(self, map):
        self.map = map

    def empty(dim_x, dim_y):
        map = np.zeros([dim_x, dim_y])
        return HydrothremalMap(map)
        
    def apply(self, vent: Vent):
        if (vent.start_x == vent.end_x):
            start = vent.start_y
            end = vent.end_y
            if (start > end):
                start = vent.end_y
                end = vent.start_y
            for index in range(start, end+1):
                self.map[vent.start_x][index] += 1

        elif (vent.start_y == vent.end_y):
            start = vent.start_x
            end = vent.end_x
            if (start > end):
                start = vent.end_x
                end = vent.start_x
            for index in range(start, end+1):
                self.map[index][vent.start_y] += 1

        else:
            dim = abs(vent.end_x - vent.start_x)
            for index in range(dim+1):
                x = vent.start_x+index
                y = vent.start_y+index
                if (vent.start_x > vent.end_x):
                    x = vent.start_x-index
                if (vent.start_y > vent.end_y):
                    y = vent.start_y-index
                self.map[x][y] += 1

    def filter(self, filter_function):
        copy = np.copy(self.map)
        for cell in np.nditer(copy, op_flags=['readwrite']):
            if (not filter_function(cell)):
                cell[...] = 0
        return HydrothremalMap(copy)

    def counts(self):
        return np.count_nonzero(self.map)
