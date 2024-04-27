import React, { useState } from "react";

function FileUploadButton(props) {
	const [selectedFile, setSelectedFile] = useState(null);

	const handleFileChange = (event) => {
		setSelectedFile(event.target.files[0]);
	};

	const handleUpload = async () => {
		if (!selectedFile) return; // Handle no file selected

		const formData = new FormData();
		formData.append("file", selectedFile);

		try {
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
			props.setFileInfo(fileListData.result);
		} catch (error) {
			console.error("Error uploading file:", error);
			// Handle upload error (e.g., display error message)
		} finally {
			setSelectedFile(null); // Clear selected file after upload
		}
	};
	return (
		<div>
			<input type="file" onChange={handleFileChange} />
			<button onClick={handleUpload}>Upload File</button>
		</div>
	);
}

export default FileUploadButton;
