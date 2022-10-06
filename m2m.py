#! /usr/bin/env python3
import pandm
import sys

if __name__ == "__main__":
    for string in sys.argv[1:]:
        majorana = pandm.MajoranaString.from_string(string)
        majorana.normalize()
        print(majorana)
