import "./App.css";
import { useEffect, useState } from "react";
import File from "./File";
import Upload from "./Upload";
function App() {
	const [fileInfo, setFileInfo] = useState([]);

	useEffect(() => {
		fetch(`http://localhost:8080/files`)
			.then((response) => {
				return response.json();
			})
			.then((data) => {
				setFileInfo(data.result);
			});
		fetch(`http://localhost:8080/api/download`);
	}, []); // No dependency array, fetches once on mount

	return (
		<div className="App">
			<div>
				{fileInfo.length > 0 ? (
					fileInfo.map((file, key) => {
						return <File fileName={file} />;
					})
				) : (
					<p>No files uploaded</p>
				)}
			</div>
			<div>
				<Upload />
			</div>
		</div>
	);
}

export default App;
