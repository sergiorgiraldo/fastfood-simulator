export async function insertOrder(order) {
	const requestOptions = {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(order)
	};

	const response = await fetch("http://localhost:8000/order", requestOptions);
	const data = await response.json();

	return JSON.stringify(data);
}
