import React from 'react';
import Head from "next/head";
import './App.css';

const TITLE = "動画ダウンロード太郎";

function HomePage() {
	return (
	<>
		<Head>
			<title>{TITLE}</title>
		</Head>
		<h1>{TITLE}</h1>
		<br/>
		<form action="/api/download" method="post">
			<h3>URLかなにか</h3>
			<input type="text" name="param" />

			<br/>
			<input type="submit" value="ダウンロード" />
		</form>
	</>
	);
}

export default HomePage;
