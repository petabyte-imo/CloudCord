import React, { useState } from "react";

function File(props) {
	const [isDownloading, setIsDownloading] = useState(false);
	const [filePath, setFilePath] = useState(""); // Replace with your actual file path
	const defaultFilename = "my_downloaded_file.txt"; // Optional default filename
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
			// Handle download error (e.g., display message to user)
		}
	}
	const downloadClick = async () => {
		setIsDownloading(true);

		try {
			await downloadFile(filePath);
		} catch (error) {
			console.error("Error downloading file:", error);
			// Handle download error (e.g., display message to user)
		} finally {
			setIsDownloading(false);
		}
	};
	return (
		<div>
			{`File Name: ${props.fileName}`}{" "}
			<button
				disabled={isDownloading}
				onClick={() => {
					setFilePath(props.fileName);
					downloadClick();
				}}
			>
				{isDownloading ? "Processing..." : "Download File"}
			</button>
		</div>
	);
}

export default File;
