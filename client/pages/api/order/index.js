import { insertOrder } from "../../../../helpers/api-util";

async function handler(req, res) {
	const data = await insertOrder(req.body);

	res.status(201).json({
		response: data,
	});
}

export default handler;