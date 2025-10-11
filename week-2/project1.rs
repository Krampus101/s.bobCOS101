fn main() {
	Let p:f64 = 520000000; // N520000000
	Let r:f64 = 10.0; // 10% per annum
	Let t:f64 = 5.0; // 5 years

	// Compound Interest
	Let a = p * ( 1.0 + (r/ 100.0))* t;
	Let compound_interest = amount - principal;

	// Output

	println!("Compound Interest (CI): N{}", compound_interest );
}