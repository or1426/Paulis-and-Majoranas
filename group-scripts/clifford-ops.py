#! /usr/bin/env python3

import pandm
import sys
from copy import deepcopy

def phase2Sign(pauli):
    if pauli.phase == 0:
        return " "
    if pauli.phase == 1:
        return "i"
    if pauli.phase == 2:
        return "-"
    if pauli.phase == 3:
        return "-i"

def fancyPauliFormatting1(pauli):
    return " ⊗ ".join(map(str, pauli.paulis))

def fancyPauliFormatting2(pauli):
    return phase2Sign(pauli) + " ⊗ ".join(map(str, pauli.paulis))


if __name__ == "__main__":
    
    for p1 in pandm.Pauli:              
        for p2 in pandm.Pauli:

            p2Pauli = pandm.PauliString(0, [p2])
            hPauli = pandm.PauliString(0, [p2])
            hPauli.H(0)
            sPauli = pandm.PauliString(0, [p2])
            sPauli.S(0)
            
            sStarPauli = pandm.PauliString(0, [p2])
            sStarPauli.S(0)
            sStarPauli.S(0)
            sStarPauli.S(0)

            
            pauli = pandm.PauliString(0, [p1,p2])

            czPauli = deepcopy(pauli)
            czPauli.CZ(0,1)

            cxPauli = deepcopy(pauli)            
            cxPauli.H(1)            
            cxPauli.CZ(0,1)            
            cxPauli.H(1)
            
            print("CX {} CX = {}".format(fancyPauliFormatting1(pauli), fancyPauliFormatting2(cxPauli)), end='\t')

            end = '\n'
            
            if p1 == pandm.Pauli.I or p1 == pandm.Pauli.X or p1 == pandm.Pauli.Y:
                end = '\t'
                
            print("CZ {} CZ = {}".format(fancyPauliFormatting1(pauli), fancyPauliFormatting2(czPauli)), end=end)

            
            if p1 == pandm.Pauli.I:
                print("h {} h = {}".format(fancyPauliFormatting1(p2Pauli), fancyPauliFormatting2(hPauli)))
            elif p1 == pandm.Pauli.X:
                print("s {} s* = {}".format(fancyPauliFormatting1(p2Pauli), fancyPauliFormatting2(sPauli)))
            elif p1 == pandm.Pauli.Y:
                print("s* {} s = {}".format(fancyPauliFormatting1(p2Pauli), fancyPauliFormatting2(sStarPauli)))
        print()
            
