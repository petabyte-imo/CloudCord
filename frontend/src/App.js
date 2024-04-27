import "./App.css";
import { useEffect, useState } from "react";
import File from "./File";
import Upload from "./Upload";
function App() {
	const [fileInfo, setFileInfo] = useState([]);

	useEffect(() => {
		fetchFileList();
	}, []); // No dependency array, fetches once on mount
	const fetchFileList = async () => {
		try {
			const response = await fetch(`http://localhost:8080/files`);
			if (!response.ok) {
				throw new Error("Failed to fetch file list");
			}
			const data = await response.json();
			setFileInfo(data.result);
		} catch (error) {
			console.error("Error fetching file list:", error);
			// Handle fetch error (e.g., display error message)
		}
	};
	return (
		<div className="App">
			<div>
				{fileInfo.length > 0 ? (
					fileInfo.map((file, key) => {
						return <File fileName={file} setFileInfo={setFileInfo} />;
					})
				) : (
					<p>No files uploaded</p>
				)}
			</div>
			<div>
				<Upload setFileInfo={setFileInfo} />
			</div>
		</div>
	);
}

export default App;
