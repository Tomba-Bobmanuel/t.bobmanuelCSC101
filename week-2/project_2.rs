fn main() {

	let t:f64 = 2.0 * 450_000.0;
	let m:f64 = 450_000.0;
	let hp:f64 = 3.0 * 450_000.0;
	let d:f64 = 3.0 * 450_000.0;
	let a:f64 = 450_000.0;

	let sum = t + m + hp + d + a;
	let avg = sum/(2.0 + 1.0 + 3.0 + 3.0 +  1.0);

	println!("The sum of sales is {}", sum);
	println!("The average of sales is {}", avg);

}