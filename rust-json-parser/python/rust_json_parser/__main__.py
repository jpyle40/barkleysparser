import sys
import os.path

from . import parse_json, parse_json_file, dumps, benchmark_performance

def main():
    if "--benchmark" in sys.argv:
        test_cases = {
            "Small": '{"key": "value"}',
            "Medium": '{"numbers": [' + ','.join(str(i) for i in range(100)) + ']}',
            "Large": '{"numbers": [' + ','.join(str(i) for i in range(1000)) + ']}',
        }

        for name, json_data in test_cases.items():
            rust_time, python_json_time, simplejson_time = benchmark_performance(json_data)

            json_ratio = python_json_time / rust_time
            simplejson_ratio = simplejson_time / rust_time

            print(f"\n{name} JSON ({len(json_data.encode('utf-8'))} bytes):")
            print(f"  Rust:             {rust_time:.6f}s")

            for label, elapsed, ratio in (
                ("Python json (C)", python_json_time, json_ratio),
                ("simplejson", simplejson_time, simplejson_ratio),
            ):
                if ratio >= 1:
                    comparison = f"Rust is {ratio:.2f}x faster"
                else:
                    comparison = f"Rust is {1 / ratio:.2f}x slower"

                print(f"  {label + ':':18} {elapsed:.6f}s  ({comparison})")

        return
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
