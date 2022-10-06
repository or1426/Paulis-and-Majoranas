#! /usr/bin/env python3
import pandm
import sys

if __name__ == "__main__":
    p = pandm.PauliString(0, [])
    flags = 0
    printMajorana = False
    if len(sys.argv) > 1 and sys.argv[1] == '-m':
        printMajorana = True
        flags += 1
           
    
    for string in sys.argv[1+flags:]:
        if any(map(str.isdigit, string)):
            #if it contains a number assume its a Majorana string
            majorana = pandm.MajoranaString.from_string(string)
            p @= majorana.to_pauli_string()
        else:
            #otherwise assume it is a Pauli
            p @= pandm.PauliString.from_string(string)
            
    if printMajorana:
        print(p.to_majorana_string())
    else:
        print(p)
