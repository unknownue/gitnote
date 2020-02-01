
from collections import OrderedDict
from collections import namedtuple
from itertools import product

class RunBuilder:
    @staticmethod
    def get_runs(params: OrderedDict):

        Run = namedtuple('Run', params.keys())

        runs = []
        for v in product(*params.values()):
            runs.append(Run(*v))
        
        return runs

