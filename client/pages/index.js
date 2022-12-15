import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";
import { useRef } from "react";
import { useEffect, useState } from "react";
// import io from "Socket.IO-client";
import { w3cwebsocket as W3CWebSocket } from "websocket";

let socket;

export default function Home() {
	const xburgerRef = useRef();
	const hotdogRef = useRef();
	const omeletteRef = useRef();
	const clientRef = useRef();

	const [wss, setWss] = useState();
	const [wsSession, setWssSession] = useState("");
	const [orderMsg, setOrderMsg] = useState("");

	useEffect(() => {
		socketInitializer();
	}, []);

	// const [input, setInput] = useState("");

	// const socketInitializer = async () => {
	// 	await fetch("/api/socket");
	// 	socket = io();

	// 	socket.on("connect", () => {
	// 		console.log("connected");
	// 	});

	// 	socket.on("update-input", (msg) => {
	// 		setInput(msg);
	// 	});
	// };

	// const onChangeHandler = (event) => {
	// 	setInput(event.target.value);
	// 	socket.emit("input-change", event.target.value);
	// };

	const handleSubmit = (event) => {
		event.preventDefault();

		const clientName = clientRef.current.value;
		const xburgerSelected = xburgerRef.current.checked;
		const hotdogSelected = hotdogRef.current.checked;
		const omeletteSelected = omeletteRef.current.checked;

		const orderData = {
			id: "",
			client: clientName,
			xburger: xburgerSelected,
			hotdog: hotdogSelected,
			omelette: omeletteSelected
		};
		fetch("/api/order", {
			method: "POST",
			body: JSON.stringify(orderData),
			headers: {
				"Content-Type": "application/json"
			}
		})
			.then((response) => response.json())
			.then((data) => {
				console.log(data);
			});
	};

	const currentWsSession = (ws_session) => {
		setWssSession(ws_session);
		tracking(ws_session);
	};

	const tracking = (ws_session) => {
		let socket = new W3CWebSocket(ws_session);

		setWss(socket);

		socket.addEventListener("open", () => {
			console.log("[websockets] Connected");
		});

		socket.addEventListener("message", (event) => {
			if (event?.data) {
				setTimeout(
					() => setOrderMsg(event.data),
					5000
				);
			}
		});

		socket.addEventListener("close", () => {
			console.log("[websockets] closed");
			setOrderMsg("");
		});
	};

	const socketInitializer = () => {
		const post_data = {
			user_id: 1
		};

		const callRegister = async () => {
			try {
				const res = await fetch(
					"/api/register",
					{
						method: 'POST',
						body: JSON.stringify(post_data)
					}
				);
				const data = await res.json();
				currentWsSession(data.url);
			} catch (err) {
				console.log(err);
			}
		};
		callRegister();		
	};

	return (
		<div className={styles.container}>
			<Head>
				<title>Create Next App</title>
				<meta name="description" content="Fstfood simulator" />
				<link rel="icon" href="/favicon.ico" />
			</Head>

			<main className={styles.main}>
				<h1 className={styles.title}>Fastfood simulator</h1>

				<div className={styles.grid}>
					<h2>Order here</h2>
					<form>
						<p className={styles.card}>
							XBurger + onion
							<input
								type="checkbox"
								name="xburger"
								id="xburger"
								ref={xburgerRef}
							/>
							<br />
							Hotdog + fries{" "}
							<input
								type="checkbox"
								name="hotdog"
								id="hotdog"
								ref={hotdogRef}
							/>
							<br />
							Omelette + salad{" "}
							<input
								type="checkbox"
								name="omelette"
								id="omelette"
								ref={omeletteRef}
							/>
							<br />
							Your name:{" "}
							<input
								type="text"
								name="client"
								id="client"
								ref={clientRef}
							/>
							<br />
							<button onClick={handleSubmit}>Do it!</button>
						</p>
					</form>

					<h2>Kitchen</h2>
					<p className={styles.card}>
						{orderMsg}
					</p>
				</div>
				{/* <input
					placeholder="Type something - websocket demo"
					value={input}
					onChange={onChangeHandler}
				/> */}
			</main>

			<footer className={styles.footer}>
				<a
					href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
					target="_blank"
					rel="noopener noreferrer">
					Powered by{" "}
					<span className={styles.logo}>
						<Image
							src="/vercel.svg"
							alt="Vercel Logo"
							width={72}
							height={16}
						/>
					</span>
				</a>
				:: Sergio Rodrigues Giraldo - &copy;2022
			</footer>
		</div>
	);
}
