#! /usr/bin/env python3
import pandm
import sys

if __name__ == "__main__":
    for string in sys.argv[1:]:
        pauli = pandm.PauliString.from_string(string)
        majorana = pauli.to_majorana_string()
        majorana.normalize()
        print(majorana)
    

