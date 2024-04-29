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
		<div className="container">
			<header className="header">
				<h1>Discord Cloud File Manager</h1>
				<div>
					<Upload setFileInfo={setFileInfo} />
				</div>
			</header>

			<table className="table">
				<thead>
					<tr>
						<th style={{ textAlign: "center" }}>Filename</th>
						<th style={{ textAlign: "center" }}>Actions</th>
					</tr>
				</thead>
				<tbody>
					{fileInfo.length > 0 ? (
						fileInfo.map((file, key) => (
							<File fileName={file} setFileInfo={setFileInfo} />
						))
					) : (
						<tr>
							<td colSpan="2">No files uploaded</td>
						</tr>
					)}
				</tbody>
			</table>
		</div>
	);
}

export default App;
