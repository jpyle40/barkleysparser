import sys
import os.path

from . import parse_json, parse_json_file, dumps


def main():
    if len(sys.argv) > 1:
        argument = sys.argv[1]

        if os.path.isfile(argument):
            result = parse_json_file(argument)
        else:
            result = parse_json(argument)

        print(dumps(result, indent=2))


if __name__ == "__main__":
    main()
