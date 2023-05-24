import argparse
import base64
import os

def from_plagr(file_name, no_cleanup=False):
    with open(file_name, 'r') as plagr_file:
        lines = plagr_file.readlines()
        ext = lines[0].strip()
        encoded_string = ''.join(lines[1:])

    decoded_string = base64.b64decode(encoded_string)

    output_file = file_name.replace('.plagr', ext)
    with open(output_file, 'wb') as original_file:
        original_file.write(decoded_string)

    if not no_cleanup:
        if os.path.exists(file_name):
            os.remove(file_name)
        backup_file = output_file + '.bak'
        if os.path.exists(backup_file):
            os.remove(backup_file)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Convert .plagr file back to original format.')
    parser.add_argument('file', help='The .plagr file to convert back.')
    parser.add_argument('--no-cleanup', action='store_true', help='Skip deleting the .plagr and .bak files after conversion.')
    args = parser.parse_args()

    from_plagr(args.file, args.no_cleanup)