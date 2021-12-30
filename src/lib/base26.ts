function toChar(num: number) {
	return String.fromCharCode('A'.charCodeAt(0) + num);
}

export function toBase26(num: number) {
	if (!Number.isInteger(num)) throw new Error("Invalid number");

	// overkill: preserve the sign of -0
	const neg = num < 0 || Object.is(num, -0);
	if (neg) num = -num;

	let result;

	if (num === 0) {
		// make sure there's one digit
		result = toChar(0);
	} else {
		// as many digits as it needs
		result = '';
		while (num > 0) {
			result = toChar(num % 26) + result;
			num = Math.floor(num / 26);
		}
	}

	// prepend the sign if negative
	if (neg) result = '-' + result;

	return result;
}
