use risc0_zkvm::guest::env;
use curve25519_dalek::edwards::{EdwardsPoint, CompressedEdwardsY};
use curve25519_dalek::scalar::Scalar;

fn main() {
	// Read two points from input
	let point1_bytes: [u8; 32] = env::read();
	let point2_bytes: [u8; 32] = env::read();

	// Decompress the points
	let point1 = CompressedEdwardsY(point1_bytes)
		.decompress()
		.expect("Invalid point 1");
	let point2 = CompressedEdwardsY(point2_bytes)
		.decompress()
		.expect("Invalid point 2");

	// Add the points
	let result = point1 + point2;

	// Commit the result to the journal
	env::commit(&result.compress().to_bytes());
}
