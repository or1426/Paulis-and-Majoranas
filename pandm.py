#! /usr/bin/env python3

from enum import Enum, auto
from dataclasses import dataclass
import unicodedata

class Pauli(Enum):
    I = auto()
    X = auto()
    Y = auto()
    Z = auto()

    def prod(p1, p2):
        # return a tuple of (phase, Pauli) resulting from p1 p2
        # recall a a = I, a b = -b a, X Y = i Z (+ cyclic permutations)
        
        if p1 == p2:
            return 0, Pauli.I
        if p1 == Pauli.I:
            return 0, p2
        if p2 == Pauli.I:
            return 0, p1
        if p1 == Pauli.X and p2 == Pauli.Y:
            return (1, Pauli.Z)
        if p1 == Pauli.Y and p2 == Pauli.X:
            return (3, Pauli.Z)
        
        if p1 == Pauli.Y and p2 == Pauli.Z:
            return (1, Pauli.X)
        if p1 == Pauli.Z and p2 == Pauli.Y:
            return (3, Pauli.X)

        if p1 == Pauli.Z and p2 == Pauli.X:
            return (1, Pauli.Y)
        if p1 == Pauli.X and p2 == Pauli.Z:
            return (3, Pauli.Y)

        #could throw an error here
        print("error:", p1, p2)
        return None
    
    def __str__(self):
        if self == Pauli.I:
            return "I"
        if self == Pauli.X:
            return "X"
        if self == Pauli.Y:
            return "Y"
        if self == Pauli.Z:
            return "Z"
        return None
                
    
@dataclass
class PauliString:
    """Represents a length N string of Pauli operators with a phase either 1, i, -1, or -i"""
    phase: int
    paulis: list[Pauli]

    def from_string(string):
        phase, string = parse_phase_from_string(string)
        newPauliString = PauliString(phase, [])
        for char in string:
            if char == "I":
                newPauliString.paulis.append(Pauli.I)
            if char == "X":
                newPauliString.paulis.append(Pauli.X)
            if char == "Y":
                newPauliString.paulis.append(Pauli.Y)
            if char == "Z":
                newPauliString.paulis.append(Pauli.Z)
        return newPauliString                

    def __matmul__(self, other):
        newPauliString = PauliString(self.phase + other.phase, [])

        full_len = max([len(self.paulis), len(other.paulis)])
        overlap_len = min([len(self.paulis), len(other.paulis)])

        bigger = other
        if len(self.paulis) > len(other.paulis):
            bigger = self
        
        for i in range(overlap_len):
            (phase, pauli) = Pauli.prod(self.paulis[i], other.paulis[i])
            newPauliString.phase += phase
            newPauliString.paulis.append(pauli)

        for i in range(overlap_len, full_len):
            newPauliString.paulis.append(bigger.paulis[i])
            
        newPauliString.phase %= 4

        return newPauliString

    def __imatmul__(self, other):
        self.phase += other.phase
        overlap_len = min([len(self.paulis), len(other.paulis)])
        
        for i in range(overlap_len):
            (phase, pauli) = Pauli.prod(self.paulis[i], other.paulis[i])
            self.phase += phase
            self.paulis[i] = pauli

        if len(other.paulis) > len(self.paulis):
            for i in range(len(self.paulis), len(other.paulis)):
                self.paulis.append(other.paulis[i])
            
        self.phase %= 4
        return self

    def from_majorana_idx(n:int):
        """
        A Majorana operator is a Pauli string
        for n = 2m even its a Z on each qubit up to m-1 followed by a Y on qubit m
        for n = 2m+1 odd its a Z on each qubit up to m-1 followed by an X on qubit m
        """
        newPauliString = PauliString(0, [])

        parity = n % 2
        qubit = (n - parity) // 2

        for i in range(qubit):
            newPauliString.paulis.append(Pauli.Z)
        if parity == 0:
            newPauliString.paulis.append(Pauli.Y)
        else:
            newPauliString.paulis.append(Pauli.X)
        return newPauliString
    
    def to_majorana_string(self):
        self_cpy = PauliString(self.phase, self.paulis.copy())

        majs = []
        for i in reversed(range(len(self_cpy.paulis))):
            if (self_cpy.paulis[i] == Pauli.X) or (self_cpy.paulis[i] == Pauli.Z):
                majs.append(2*i+1)
                self_cpy @= PauliString.from_majorana_idx(2*i+1)

            if (self_cpy.paulis[i] == Pauli.Y):
                majs.append(2*i)
                self_cpy @= PauliString.from_majorana_idx(2*i)

        return MajoranaString(self_cpy.phase, list(reversed(majs)))

    def __str__(self):
        s = None
        if self.phase == 0:
            s = ""
        if self.phase == 1:
            s = "+i"
        if self.phase == 2:
            s = "-"
        if self.phase == 3:
            s = "-i"
        for p in self.paulis:
            s += str(p)
        return s

    def H(self, idx):
        if idx < len(self.paulis):
            if self.paulis[idx] == Pauli.X:
                self.paulis[idx] = Pauli.Z
            elif self.paulis[idx] == Pauli.Y:
                self.phase += 2
                self.phase %= 4
            elif self.paulis[idx] == Pauli.Z:
                self.paulis[idx] = Pauli.X
    def S(self, idx):
        if idx < len(self.paulis):
            if self.paulis[idx] == Pauli.X:
                self.paulis[idx] = Pauli.Y
                self.phase += 2
                self.phase %= 4
            elif self.paulis[idx] == Pauli.Y:
                self.paulis[idx] = Pauli.X
    def CZ(self, i1, i2):
        m = max([i1,i2])
        if m >= len(self.paulis):
            for _ in range(len(self.paulis), m+1):
                self.paulis.append(Pauli.I)
        if self.paulis[i1] == Pauli.I and self.paulis[i2] == Pauli.X:
            self.paulis[i1] = Pauli.Z
        elif self.paulis[i1] == Pauli.I and self.paulis[i2] == Pauli.Y:
            self.paulis[i1] = Pauli.Z            
        elif self.paulis[i1] == Pauli.X and self.paulis[i2] == Pauli.I:
            self.paulis[i2] = Pauli.Z
        elif self.paulis[i1] == Pauli.X and self.paulis[i2] == Pauli.X:
            self.paulis[i1] = Pauli.Y
            self.paulis[i2] = Pauli.Y
        elif self.paulis[i1] == Pauli.X and self.paulis[i2] == Pauli.Y:
            self.paulis[i1] = Pauli.Y
            self.paulis[i2] = Pauli.X
            self.phase += 2
            self.phase %= 4
        elif self.paulis[i1] == Pauli.X and self.paulis[i2] == Pauli.Z:
            self.paulis[i2] = Pauli.I
        elif self.paulis[i1] == Pauli.Y and self.paulis[i2] == Pauli.I:
            self.paulis[i2] = Pauli.Z
        elif self.paulis[i1] == Pauli.Y and self.paulis[i2] == Pauli.X:
            self.paulis[i1] = Pauli.X
            self.paulis[i2] = Pauli.Y
            self.phase += 2
            self.phase %= 4
        elif self.paulis[i1] == Pauli.Y and self.paulis[i2] == Pauli.Y:
            self.paulis[i1] = Pauli.X
            self.paulis[i2] = Pauli.X
        elif self.paulis[i1] == Pauli.Y and self.paulis[i2] == Pauli.Z:
            self.paulis[i2] = Pauli.I        
        elif self.paulis[i1] == Pauli.Z and self.paulis[i2] == Pauli.X:
            self.paulis[i1] = Pauli.I
        elif self.paulis[i1] == Pauli.Z and self.paulis[i2] == Pauli.Y:
            self.paulis[i1] = Pauli.I
        

@dataclass
class MajoranaString:
    phase: int
    majs: list[int]
                
    def to_pauli_string(self):
        newPauliString = PauliString(self.phase, [])
        #print(list(self.majs))
        for maj in self.majs:
            #print(maj)
            #print(PauliString.from_majorana_idx(maj))
            newPauliString @= PauliString.from_majorana_idx(maj)
            
        return newPauliString
    def __str__(self, symbol=None):
        if symbol == None:
            symbol = "c"
        s = None
        if self.phase == 0:
            s = ""
        if self.phase == 1:
            s = "+i"
        if self.phase == 2:
            s = "-"
        if self.phase == 3:
            s = "-i"
        return s + " ".join(["{}{}".format(symbol,idx) for idx in self.majs])

    def normalize(self):
        """
        turn it into canonical form
        """

        def bubbleSort(l):
            swaps = 0
            for i in range(len(l)): 
                swapped = False        
                for j in range(0, len(l) - i - 1):            
                    if l[j] > l[j + 1]:
                        l[j], l[j+1] = l[j+1], l[j]
                        swapped = True
                        swaps += 1
          
                if not swapped:
                    break
            return (2 * swaps) % 4
        
        self.phase += bubbleSort(self.majs)
        self.phase %= 4
        
        while True:
            changes = False
            for i in range(len(self.majs) - 1):
                if self.majs[i] == self.majs[i+1]:
                    self.majs.pop(i)
                    self.majs.pop(i)
                    changes = True
                    break
            if not changes:
                break
        return self
    
    def from_string(string):
        phase, string = parse_phase_from_string(string)
        newMajString = MajoranaString(phase, [])
        
        inInt = False
        int_start = None
        for i, char in enumerate(string):
            if not inInt:
                if not char.isdigit():
                    pass
                else:
                    inInt = True
                    int_start = i
            else:
                if not char.isdigit():
                    inInt = False
                    newMajString.majs.append(int(string[int_start:i]))
                else:
                    pass
        if inInt:
            newMajString.majs.append(int(string[int_start:]))
        return newMajString


def parse_phase_from_string(string):
    """
    look for a string with prefix out of
    {+, +i, i, -, -i}
    return interpretation of prefix as a number k (where (i)^k is the phase) as well as the rest of the string
    """

    fixed_prefixes = {'+': 0,
                      '+i': 1,
                      'i': 1,
                      '-': 2,
                      '-i': 3}
    if string[:2] in fixed_prefixes:
        return fixed_prefixes[string[:2]], string[2:]
    if string[:1] in fixed_prefixes:
        return fixed_prefixes[string[:1]], string[1:]
    
    #if there is no prefix assume the phase is 1
    return (0, string)

    
