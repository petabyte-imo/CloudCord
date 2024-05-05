import React, { useState } from "react";

function File(props) {
	const [isDownloading, setIsDownloading] = useState(false);
	const [isDeleting, setIsDeleting] = useState(false);
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
	const deleteClick = () => {
		setIsDeleting(true);
		fetch(`http://localhost:8080/api/delete/${filePath}`, {
			method: "POST",
		})
			.then((response) => {
				if (!response.ok) {
					throw new Error("Failed to delete file");
				}
				// Fetch the file list again after deletion
				return fetch(`http://localhost:8080/files`);
			})
			.then((response) => response.json())
			.then((data) => {
				// Update the file list in the parent component
				props.setFileInfo(data.result);
			})
			.catch((error) => {
				console.error("Error deleting file:", error);
				// Handle delete error (e.g., display message to user)
			})
			.finally(() => {
				setIsDeleting(false);
			});
	};
	const encrypted = props.encrypted;
	return (
		<tr>
			<td className="fileName">
				{`${props.fileName}`} (
				{encrypted === "true" ? "Encrypted" : "Not Encrypted"})
			</td>
			<td className="buttonContainer">
				<button
					className="downloadBtn"
					disabled={isDownloading}
					onClick={() => {
						setFilePath(props.fileName);
						downloadClick();
					}}
				>
					{isDownloading ? "Processing..." : "Download File"}
				</button>
				<button
					className="deleteBtn"
					disabled={isDeleting}
					onClick={() => {
						setFilePath(props.fileName);
						deleteClick();
					}}
				>
					{isDeleting ? "Processing..." : "Delete File"}
				</button>
			</td>
		</tr>
	);
}

export default File;
