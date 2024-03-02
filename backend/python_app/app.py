from utils.label import print_label
from urllib.parse import parse_qsl


def run(*args):
    # print(args)

    params_dict = dict(parse_qsl(args[1]))

    print(params_dict)

    return print_label(args[0], params_dict)
