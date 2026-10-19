import sys
import os.path

from . import parse_json, parse_json_file, dumps


def main():
    if len(sys.argv) > 1:
        argument = sys.argv[1]

        try:
            if os.path.isfile(argument):
                result = parse_json_file(argument)
            else:
                result = parse_json(argument)

            print(dumps(result, indent=2))

        except (ValueError, IOError) as error:
            print(f"Error: {error}", file=sys.stderr)
            sys.exit(1)


main()
