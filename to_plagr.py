import argparse
import base64
import os
import shutil

def to_plagr(file_name, optional_ext=None, no_backup=False):
    ext = optional_ext if optional_ext else os.path.splitext(file_name)[1]
    with open(file_name, 'rb') as original_file:
        encoded_string = base64.b64encode(original_file.read()).decode('utf-8')

    # Make a backup copy of the file unless the no_backup flag is set
    if not no_backup:
        shutil.copyfile(file_name, file_name + '.bak')
    
    with open(file_name + '.plagr', 'w') as plagr_file:
        plagr_file.write(ext + '\n')  
        plagr_file.write(encoded_string)  

    # Remove the original file and rename the .plagr file
    os.remove(file_name)
    os.rename(file_name + '.plagr', file_name.replace(os.path.splitext(file_name)[1], '.plagr'))

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Convert file to .plagr format.')
    parser.add_argument('file', help='The file to convert.')
    parser.add_argument('--ext', help='Optional new file extension for when converting back from .plagr')
    parser.add_argument('--no-backup', action='store_true', help='Skip creating a backup of the original file.')
    args = parser.parse_args()

    to_plagr(args.file, args.ext, args.no_backup)