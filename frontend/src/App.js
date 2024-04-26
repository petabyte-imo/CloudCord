import fileInfo from "./File";
import "./App.css";
import { useState } from "react";

function App() {
	const [isDownloading, setIsDownloading] = useState(false);
	const [fileInfo, setFileInfo] = useState([]);
	const filePath = "@Jyoeenk - Discord 2023-12-03 21-49-05.mp4"; // Replace with your actual file path
	const defaultFilename = "my_downloaded_file.txt"; // Optional default filename

	const downloadClick = async () => {
		setIsDownloading(true);
		try {
			await downloadFile(filePath);
		} catch (error) {
			console.error("Error downloading file:", error);
			// Handle download errors (e.g., display a message to the user)
		} finally {
			setIsDownloading(false);
		}
	};

	async function downloadFile(filePath, filename = "") {
		try {
			const response = await fetch(
				`http://localhost:8080/api/download/${filePath}`
			);

			if (!response.ok) {
				throw new Error(`Download failed with status: ${response.status}`);
			}

			const blob = await response.blob();
			const url = window.URL.createObjectURL(blob);
			const link = document.createElement("a");
			link.href = url;
			link.setAttribute("download", filename || filePath.split("/").pop());
			document.body.appendChild(link);
			link.click();
			document.body.removeChild(link);
			window.URL.revokeObjectURL(url);
		} catch (error) {
			console.error("Error downloading file:", error);
		}
	}

	return (
		<div className="App">
			<button disabled={isDownloading} onClick={downloadClick}>
				{isDownloading ? "Downloading..." : "Download File"}
			</button>
		</div>
	);
}

export default App;
