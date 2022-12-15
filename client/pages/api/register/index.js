const RegisterHandler = async (req, res) => {
	try {
		let request_url = "http://localhost:8000/register";

		// This is a contrived example, normally your external API would exist on another domain.
		const response = await fetch(request_url, {
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			},
			body: req.body
		}).catch((error) => {
			console.error("Error:", error);
		});

		const url_res = await response.json();

		console.log(url_res);

		res.status(response.status || 200).json(url_res);
	} catch (error) {
		console.error(error);
		res.status(error.status || 500).json({
			code: error.code,
			error: error.message
		});
	}
};

export default RegisterHandler;
