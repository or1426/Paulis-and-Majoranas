pub use num::complex::Complex;
use num::complex::ComplexFloat;

use std::fmt;

use std::collections::HashMap;
use std::str::FromStr;

use std::ops::Mul;
use std::ops::Add;
use std::ops::AddAssign;

use std::iter::Product;
use std::iter::Sum;

#[derive(Clone)]
pub struct MajoranaExpr {
    map : HashMap<u64, Complex<f64> >
}

impl MajoranaExpr {
    pub fn from_str(s: &str) -> MajoranaExpr {
	match Complex::<f64>::from_str(s) {
	    Ok(num) => MajoranaExpr {
		map : HashMap::from([(0, num)])
	    },
	    _ => {
		let v: Vec<&str> = s.rmatches(char::is_numeric).collect();
		MajoranaExpr {
		    map : HashMap::from([(1 << u64::from_str(v[0]).unwrap(), Complex::<f64>::new(1.,0.))])
		}
	    }
	}
    }

    pub fn new() -> MajoranaExpr {
	MajoranaExpr {
	    map : HashMap::from([])
	}
    }

    fn remove_zeros(&mut self) {
	let mut to_remove : Vec<u64> = Vec::new();
	for key in self.map.keys() {
	    if self.map[key].abs() < 1e-14 {
		to_remove.push(*key);
	    }
	}

	for key in to_remove.iter(){
	    self.map.remove(key);
	}
    }
}

impl fmt::Display for MajoranaExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let mut idx = 0;
	if self.map.len() == 0 {
	    return write!(f, "0");
	}
	for key in self.map.keys() {
	    match write!(f, "({})", self.map[key]) {
		Err(error) => {return Err(error)},
		_ => {}
	    }
	    
	    for i in 0..64 {
		if ((key >> i) & 1) == 1{
		    match write!(f, "c{}", i) {
			Err(error) => {return Err(error)},
			_ => {}
		    }
		}
	    }

	    if idx != self.map.len() - 1 {
		match write!(f, " + ") {
		    Err(error) => {return Err(error)},
		    _ => {}
		}
	    }
	    idx += 1;
	}
	return Result::Ok(());
    }
}


//we count the number of swaps required to simplify c(x)c(y) mod 2
// so c(x)c(y) = (-1)^prod(x,y) c(x + y)
fn prod(x: &u64 , y: &u64) -> bool {
    let mut mask : u64 = !0; //mask initially contains all 1s
    let mut count = 0;
    for i in 0..64{
	mask ^= 1 << i;
	if ((y >> i) & 1) == 1 {
	    count += (mask & x).count_ones();
	}
	//println!("{} {} {}", x,y,count);
    }
    if (count % 2) == 1 {
	return true;
    }
    return false;
}

impl Mul<MajoranaExpr> for MajoranaExpr {
    type Output = MajoranaExpr;

    fn mul(self, rhs: MajoranaExpr) -> Self::Output {
        let mut prod_expr = MajoranaExpr::new();

	for k1 in self.map.keys(){
	    for k2 in rhs.map.keys(){
		let coeff = self.map[k1] * rhs.map[k2] * match prod(k1,k2) {
		    true => -1.,
		    false => 1.
		};
		//println!("{} + {} = {}", k1,k2, prod(k1,k2));
		*prod_expr.map.entry(k1 ^ k2).or_insert(Complex::new(0.,0.)) += coeff;		
	    }
	    prod_expr.remove_zeros();
	}	
	return prod_expr;
    }
}

impl Mul<Complex<f64> > for MajoranaExpr {
    type Output = MajoranaExpr;

    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        let mut prod_expr = self.clone();

	for val in prod_expr.map.values_mut() {
	    *val *= rhs;
	}
	return prod_expr;
    }
}

impl Mul<MajoranaExpr> for Complex<f64> {
    type Output = MajoranaExpr;

    fn mul(self, rhs: MajoranaExpr) -> Self::Output {
        let mut prod_expr = rhs.clone();

	for val in prod_expr.map.values_mut() {
	    *val *= self;
	}
	return prod_expr;
    }
}


impl Add<MajoranaExpr> for MajoranaExpr {
    type Output = MajoranaExpr;

    fn add(self, rhs: MajoranaExpr) -> Self::Output {
        let mut sum_expr = self.clone();

	for k in rhs.map.keys(){
	    *sum_expr.map.entry(*k).or_insert(Complex::new(0.,0.)) += rhs.map[k];
	}
	sum_expr.remove_zeros();
	return sum_expr;
    }
}

impl AddAssign for MajoranaExpr {
    fn add_assign(&mut self, rhs: Self) {
        for k in rhs.map.keys(){
	    *self.map.entry(*k).or_insert(Complex::new(0.,0.)) += rhs.map[k];
	}
	self.remove_zeros();
    }
}





impl Sum<MajoranaExpr> for MajoranaExpr
{
    fn sum<I>(iter: I) -> MajoranaExpr
    where
        I: Iterator<Item = MajoranaExpr>,
    {
        let mut acc = MajoranaExpr::new();

	for val in iter {
	    acc += val;
	}
	return acc;
    }
}



impl Product<MajoranaExpr> for MajoranaExpr
{
    fn product<I>(iter: I) -> MajoranaExpr
    where
        I: Iterator<Item = MajoranaExpr>,
    {
	let mut acc = MajoranaExpr::new();
	acc.map.insert(0,  Complex::new(1.,0.));

	for val in iter {
	    acc = acc*val;
	}
        return acc;
    }
}

