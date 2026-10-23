import sys
import os.path

from . import parse_json, parse_json_file, dumps


def main():
    if len(sys.argv) == 1:
        if not sys.stdin.isatty():
            try:
                data = sys.stdin.read()
                result = parse_json(data)
                print(dumps(result, indent=2))
                return
            except ValueError as error:
                print(f"Error: {error}", file=sys.stderr)
                sys.exit(1)

        print("Usage: python -m rust_json_parser <file_or_json>")
        sys.exit(1)    
    if sys.argv[1] in ("--help", "-h"):
        print("Usage: python -m rust_json_parser <file_or_json>")
        sys.exit(0)
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
