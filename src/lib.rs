use num_bigint::{BigUint};
struct EllipticCurve {
// y**2 = x ** 2 + a*x + b
a: BigUint,
b: BigUint,
prime: BigUint
}

struct Point {
    x: BigUint,
    y: BigUint
}


struct FiniteField {}

impl FiniteField {

    fn add(p: &BigUint,q: &BigUint, prime: &BigUint)-> BigUint{
        // p+q mod prime
        todo!()
    }
    fn mul(p: &BigUint,q: &BigUint, prime: &BigUint)-> BigUint{
        // p*q mod prime
        todo!()
    }
    fn inv_addition(p: &BigUint,q: &BigUint, prime: &BigUint)-> BigUint{
        // -p mod prime
        todo!()
    }
    fn inv_mul(p: &BigUint,q: &BigUint, prime: &BigUint)-> BigUint{
        // p**(-1) mod prime
        todo!()
    }
    
}

impl EllipticCurve {

    fn add(p: &Point,q: &Point)-> Point{
        todo!()
    }

    fn doubl(p: &Point)-> Point{
        todo!()
    }

    fn scalar_mul(a: &Point,b: &BigUint)-> Point{
        todo!()
    }

    
}

