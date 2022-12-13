import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";
import { useRef } from "react";

export default function Home() {
	const xburgerRef = useRef();
	const hotdogRef = useRef();
	const omeletteRef = useRef();
	const clientRef = useRef();

	const handleSubmit = (event) => {
		event.preventDefault();

		const clientName = clientRef.current.value;
		const xburgerSelected = xburgerRef.current.checked;
		const hotdogSelected = hotdogRef.current.checked;
		const omeletteSelected = omeletteRef.current.checked;

		const orderData = { 
			client: clientName,
			xburger: xburgerSelected,
			hotdog: hotdogSelected,
			omelette: omeletteSelected
		 };
		 fetch("/api/order", {
			method: "POST",
			body: JSON.stringify(orderData),
			headers: {
				"Content-Type": "application/json",
			},
		})
		.then((response) => response.json())
		.then((data) => {
			console.log(data);
			});
	};

	return (
		<div className={styles.container}>
			<Head>
				<title>Create Next App</title>
				<meta
					name="description"
					content="Fstfood simulator"
				/>
				<link rel="icon" href="/favicon.ico" />
			</Head>

			<main className={styles.main}>
				<h1 className={styles.title}>
					Fastfood simulator
				</h1>

				<div className={styles.grid}>
						<h2>Order here</h2>
							<form>
								<p className={styles.card}>
									XBurger + onion<input type="checkbox" name="xburger" id="xburger" ref={xburgerRef}/><br/>
									Hotdog + fries <input type="checkbox" name="hotdog" id="hotdog" ref={hotdogRef}/><br/>
									Omelette + salad <input type="checkbox" name="omelette" id="omelette" ref={omeletteRef}/><br/>
									Your name: <input type="text" name="client" id="client" ref={clientRef}/><br/>
									<button onClick={handleSubmit}>Do it!</button>
								</p>
							</form>

						<h2>Orders list</h2>
						<p className={styles.card}>
						Asset csystems BATF Blowpipe Soviet South Africa wire transfer. NSA event
security Compsec spies benelux Sears Tower airframe red noise. Commecen Steve <br/>
						Asset csystems BATF Blowpipe Soviet South Africa wire transfer. NSA event
security Compsec spies benelux Sears Tower airframe red noise. Commecen Steve <br/>
						Asset csystems BATF Blowpipe Soviet South Africa wire transfer. NSA event
security Compsec spies benelux Sears Tower airframe red noise. Commecen Steve
						</p>
				</div>
				<div className={styles.grid}>
						<h2>Kitchen</h2>
						<p className={styles.card}>
						I went to the woods because I wished to live deliberately, to front only the
essential facts of life, and see if I could not learn what it had to teach, and <br/>
						I went to the woods because I wished to live deliberately, to front only the
essential facts of life, and see if I could not learn what it had to teach, and <br/>
						I went to the woods because I wished to live deliberately, to front only the
essential facts of life, and see if I could not learn what it had to teach, and
						</p>

						<h2>Ready to pickup</h2>
						<p className={styles.card}>
						I went to the woods because I wished to live deliberately, to front only the
essential facts of life, and see if I could not learn what it had to teach, and <br/>
						I went to the woods because I wished to live deliberately, to front only the
essential facts of life, and see if I could not learn what it had to teach, and <br/>
						I went to the woods because I wished to live deliberately, to front only the
essential facts of life, and see if I could not learn what it had to teach, and
						</p>
				</div>
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
				:: 
				Sergio Rodrigues Giraldo - &copy;2022
			</footer>
		</div>
	);
}
