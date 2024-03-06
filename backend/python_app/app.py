from utils.label import print_label
import json


def run(*args):
    data_dict = json.loads(args[0])
    return print_label(data_dict)
