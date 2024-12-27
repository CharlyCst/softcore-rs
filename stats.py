import os

def count_non_empty_lines(file_path):
    """Counts non-empty lines in a given file."""
    with open(file_path, 'r') as file:
        lines = file.readlines()
    return sum(1 for line in lines if line.strip())  # Count non-empty lines

def search_and_count_lines(root_dir):
    """Searches recursively for .ml and .mli files and counts non-empty lines."""
    total_non_empty_lines = 0

    # Walk through the directory
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            # Check if the file has a .ml or .mli extension
            if file.endswith('.ml') or file.endswith('.mli'):
                file_path = os.path.join(root, file)
                total_non_empty_lines += count_non_empty_lines(file_path)

    return total_non_empty_lines

if __name__ == '__main__':
    print(f"Total non-empty lines across all .ml and .mli files: {search_and_count_lines('./')}")
    print(f"Total non-empty lines in the rust code: {count_non_empty_lines("src/lib.rs")}")
    print(f"Total non-empty lines in the rust prelude: {count_non_empty_lines("sail_prelude/src/lib.rs")}")



