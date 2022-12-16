import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";
import { useEffect, useState, useRef } from "react";
import { w3cwebsocket as W3CWebSocket } from "websocket";

export default function Home() {
	const xburgerRef = useRef();
	const hotdogRef = useRef();
	const omeletteRef = useRef();
	const clientRef = useRef();

	const [orderMsg, setOrderMsg] = useState("");
	
	const effectRan = useRef(false);
	useEffect(() => {
		if (!effectRan.current) {
			initializeSocket();
		}
		return () => {effectRan.current = true};
	}, []);

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
				console.log("order:" + data.response);
			});
	};

	const trackUpdates = (ws_session) => {
		let socket = new W3CWebSocket(ws_session);

		socket.addEventListener("open", () => {
			console.log("[websockets] Connected");
		});

		socket.addEventListener("message", (event) => {
			if (event?.data) {
				setTimeout(() => setOrderMsg(event.data), 1000);
			}
		});

		socket.addEventListener("close", () => {
			console.log("[websockets] closed");
			setOrderMsg("");
		});
	};

	const initializeSocket = async () => {
		const post_data = {
			user_id: 1
		};
		console.log("Initializing");
		try {
			const res = await fetch("/api/register", {
				method: "POST",
				body: JSON.stringify(post_data)
			});
			const data = await res.json();
			trackUpdates(data.url);
		} catch (err) {
			console.log(err);
		}
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
					<p className={styles.card}>{orderMsg}</p>
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
