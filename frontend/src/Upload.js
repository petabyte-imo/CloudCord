import React, { useContext } from "react";
import { AppContext } from "./App";

function FileUploadButton({ setFileInfo }) {
	const { selectedFile, setSelectedFile } = useContext(AppContext);

	const handleFileChange = (event) => {
		setSelectedFile(event.target.files[0]);
	};

	const handleUpload = async () => {
		if (!selectedFile) return; // Handle no file selected

		const formData = new FormData();
		formData.append("file", selectedFile);

		try {
			const checkbox = document.getElementById("selectEncrypted");
			let checked = checkbox.checked;

			fetch("http://localhost:8080/set_encrypted", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(checked),
			});
			const inputBox = document.getElementById("selectEncryptKey");
			let key = inputBox.value;

			fetch("http://localhost:8080/set_encrypted_key", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(key),
			});
			const response = await fetch("http://localhost:8080/upload", {
				method: "POST",
				body: formData,
			});

			if (!response.ok) {
				throw new Error(`Upload failed with status: ${response.status}`);
			}

			console.log("File uploaded successfully!");

			// Fetch the file list again after upload
			const fileListResponse = await fetch(`http://localhost:8080/files`);
			if (!fileListResponse.ok) {
				throw new Error("Failed to fetch file list after upload");
			}

			const fileListData = await fileListResponse.json();
			// Update the file list in the parent component
			setFileInfo(fileListData);
		} catch (error) {
			console.error("Error uploading file:", error);
			// Handle upload error (e.g., display error message)
		} finally {
			setSelectedFile(null); // Clear selected file after upload
		}
	};

	return (
		<div>
			<label htmlFor="selectEncrypted">Encrypted</label>
			<input type="checkbox" id="selectEncrypted" />
			<label htmlFor="selectEncryptKey">| Encryption Key</label>
			<input
				type="text"
				id="selectEncryptKey"
				maxLength="32" // Set maximum length to 32 bytes
			/>

			<input
				type="file"
				id="selectInput"
				onChange={handleFileChange}
				style={{ display: "none" }}
			/>
			<label htmlFor="selectInput" className="input-file-label">
				Choose File
			</label>
			<button onClick={handleUpload} className="uploadButton">
				Upload File
			</button>
		</div>
	);
}

export default FileUploadButton;
