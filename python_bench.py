import timeit


setup = '''
import numpy as np

size = {}
zeros = np.zeros(tuple([2]*size))
ones = np.ones(tuple([2]*size))
'''

for size in range(2, 15, 2):
    print('Benchmark:' + str(size))
    time_taken = timeit.Timer('zeros*ones', setup=setup.format(size)).repeat(10, 1000000)
    per_loop = min(time_taken) / 1000000
    print(str(per_loop*1000000) + " us")