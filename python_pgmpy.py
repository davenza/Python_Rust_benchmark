import timeit


setup = '''
import numpy as np
from pgmpy.factors.discrete import DiscreteFactor

size = {}
domain1 = ["x" + str(index) for index in range(size)]
domain2 = ["x" + str(index) for index in range(int(size/2), int(size/2) + size)]
phi1 = DiscreteFactor(domain1, [2]*size, np.arange(2**size, dtype=np.float64))
phi2 = DiscreteFactor(domain2, [2]*size, np.arange(2**size, dtype=np.float64))
'''

for size in range(2, 13, 2):
    time_taken = timeit.Timer('phi1.product(phi2, inplace=False)', setup=setup.format(size)).repeat(10, 10000)
    per_loop = min(time_taken) / 10000
    print('Benchmark ' + str(size) + ": " + str(per_loop*1000000) + " us")