# Paulis-and-Majoranas
scripts for basic calculations with n qubit Pauli and Majorana Fermion operators. Note we use the following convention for Majorana Fermion operators. For each nonnegative integer $i$ define

$$c_{2i} = \left(\prod_{j=0}^{i-1}Z_j\right) Y_i$$ 

and 

$$c_{2i+1} = \left(\prod_{j=0}^{i-1}Z_j\right) X_i.$$

If you use this for anything, feel free to drop me an email about it, I'd be very interested to hear from you :).

# Summary

## Algebra calculator
A calculator for doing basic math in the compelx algebra generated by (up to) $64$ generators which anticommute and square to $1$. Wrapped in a basic REPL for convenience. Currently the only understood operators are `*` and `+`. The generators are labelled in the form `cN` where N is a natural number less than 64.

Example usage:
Multiplication symbol is implicit
```
>>> c0 c1 + c1*c0
0
```
We can nest expressions in  brackets and everything should get recursively evaluated
```
>> (2i + c1 c0) * (c1 * c2 + (-1 + 3i)*c9)
(-6-2i)c9 + (0+2i)c1c2 + (1-3i)c0c1c9 + (-1+0i)c0c2
```
Note that we set every complex number with abolute value less than $10^{-14}$ to $0$. Complex numbers are rust Complex\<f64\> objects.

## Group scripts
A set of simple python scripts for doing calculations in the group generated by the n qubit Pauli operators. Group multiplication, sorting to canonical form and swapping between different generating sets.

### pandm.py
Package the scripts depend on, doesn't do anything if you execute it. Maybe could add some tests to it. In addition to the functionality used in the remaining scripts it has the capability to apply Clifford operations to Pauli strings.

### clifford-ops.py

Prints out a table summarising the action of the basic Clifford operations on 1 and 2 qubit Pauli strings. Currently takes no arguments

### m2m.py 
Using the anticommutation relation $\\{c_i, c_j\\} = 2 \delta_{ij} I$, sort strings of Majorana Fermion operators into canonical order and remove duplicates.

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

### m2p.py 
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

### p2m.py
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

### prod.py

Takes one or more strings, separated by spaces, interprets each as either 1. a Majorana Fermion string if it contains at least one number, 2. a Pauli string otherwise. If it is a Majorana string it is interpreted according to the same rules as `m2m.py` and `m2p.py`, if it is a Pauli is it interpreted in the same was as `p2m.py`. The inputted strings are multiplied together and the result printed. Optional argument "-m" sets the output to be printed as a (canonical form) Majorana string, by default the output is as a Pauli string.

Example usage:
```
$ ./prod.py Z X
$ +iY
$ ./prod.py -c -iZ X
$ +c0
$ ./prod.py Z c2 IIZ
$ +IYZ
$ ./prod.py c0c1c2 c1c2c5 c4c7c2
$ -YXIX

```
