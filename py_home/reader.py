import csv
from pathlib import Path
from typing import Iterator


def gen_iterate(path:Path)->Iterator[list[str]]:
    with open(path) as file:
        reader=csv.reader(file,delimiter='\t')
        next(reader)

        for row in reader:
            yield row


