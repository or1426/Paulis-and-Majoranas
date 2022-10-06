# Paulis-and-Majoranas
scripts for basic calculations with n qubit Pauli and Majorana Fermion operators. Note we use the following convention for Majorana Fermion operators. For each nonnegative integer $i$ define

$$c_{2i} = \left(\prod_{j=0}^{i-1}Z_j\right) Y_i$$ 

and 

$$c_{2i+1} = \left(\prod_{j=0}^{i-1}Z_j\right) X_i.$$

# Summary
## pandm.py
Package the scripts depend on, doesn't do anything if you execute it. Maybe could add some tests to it

## m2m.py 
Using the anticommutation relation $\{c_i, c_j\} = 2 \delta_{ij} I$, sort strings of Majorana Fermion operators into canonical order and remove duplicates.

Input format of Majorana Fermion operators is a prefix to describe the phase, which must be one of `+`, `+i`, `i`, `-`, `-i` or the empty string followed by one or more Majorana Fermion operators. Each Majorana Fermion operator is a number, and they are seperated by any strings that arent numbers. 

Example usage:
```
$ ./m2m.py +c0c1
$ +c0 c1
$ ./m2m.py "+i0 1 2"
$ +ic0 c1 c2
$ ./m2m.py "-ic1 c0"
$ +ic0 c1
$ ./m2m.py -ic1 c0
$ -ic1
$ +c0
```

You can see you can either write the Majorana operators as cn for an integer n, or just write the integers separated by spaces, but if they are separated by spaces you have to put them in quotation marks or they will be interpreted as separate Majoranas to sort.

## m2p.py 
Takes input exactly the same as `m2m.py`, but instead of returning the canonical form for a Majorana string instead writes it as a product of Pauli operators.

Example usage:
```
$ ./m2p.py +c0c1
$ -iZ
$ ./m2p.py +c1c0c3
$ +iIX
$ ./m2p.py 27
$ +ZZZZZZZZZZZZZX
$ ./m2p.py +ic14c27
$ -IIIIIIIXZZZZZX
$ ./m2p.py +ic14 c27
$ +iZZZZZZZY
$ +ZZZZZZZZZZZZZX
```

## p2m.py
Take one or more strings of Pauli operators and write each as a string of Majoranas (in canconical form). Each input consists of a phase prefix which is the same as for `m2m.py` followed by a sequence of characters. Characters "I", "X", "Y", "Z" in this string will be interpreted as Pauli operators, other characters will be ignored. If unquoted, spaces will make breaks between different Pauli strings.

Example usage:
```
$ ./p2m.py -iXZ
$ +c1 c2 c3
$ ./p2m.py +XXZ
$ +c0 c3 c4 c5
$ ./p2m.py -iZZ +XZZ
$ +ic0 c1 c2 c3
$ -c1 c2 c3 c4 c5
```

## prod.py

Takes one or more strings, separated by spaces, interprets each as either 1. A Majorana Fermion string if it contains at least one number, a Pauli string otherwise. If it is a Majorana string it is interpreted according to the same rules as `m2m.py` and `m2p.py`, if it is a Pauli is it interpreted in the same was as `p2m.py`. The inputted strings are multiplied together and the result printed. Optional argument "-m" sets the output to be printed as a (canonical form) Majorana string, by default the output is as a Pauli string.

Example usage:
```
$ ./prod.py Z X
$ +iY
$ ./prod.py -c -iZ X
$ +c0
$ ./prod.py Z c2 IIZ
$ +IYZ
```
