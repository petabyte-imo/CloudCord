import Modal from "./Modal";
import { useContext } from "react";
import { AppContext } from "./App";
function File(props) {
	const {
		isDownloading,
		setIsDownloading,
		isDeleting,
		setIsDeleting,
		filePath,
		setFilePath,
		encryptionKey,
		setIsOpen,
		setFileInfo,
		setEncryptionBool,
		encryptionBool,
	} = useContext(AppContext);
	const deleteClick = () => {
		setIsDeleting(true);
		fetch(`http://localhost:8080/api/delete/${filePath}`, {
			method: "POST",
		})
			.then((response) => {
				if (!response.ok) {
					throw new Error("Failed to delete file");
				}
				setTimeout(() => {
					setIsDeleting(true);
				}, 3000);
				// Fetch the file list again after deletion
				return fetch(`http://localhost:8080/files`);
			})
			.then((response) => response.json())
			.then((data) => {
				// Update the file list in the parent component
				setFileInfo(data.result);
			})
			.catch((error) => {
				console.error("Error deleting file:", error);
				// Handle delete error (e.g., display message to user)
			})
			.finally(() => {
				setIsDeleting(false);
			});
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
			// Handle download error (e.g., display message to user)
		}
	}
	const downloadClick = async () => {
		setIsDownloading(true);

		try {
			let checked = encryptionBool;
			console.log(checked);

			await fetch("http://localhost:8080/set_encrypted", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(checked),
			});

			let key = encryptionKey;

			fetch("http://localhost:8080/set_encrypted_key", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(key),
			});

			await downloadFile(filePath);
		} catch (error) {
			console.error("Error downloading file:", error);
			// Handle download error (e.g., display message to user)
		} finally {
			setIsDownloading(false);
		}
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
						// downloadClick();

						if (encrypted === "true") {
							setEncryptionBool(true);
							setIsOpen(true);
							downloadClick();
						} else {
							setEncryptionBool(false);
							setIsOpen(false);
							downloadClick();
						}
					}}
				>
					Download File
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
			<Modal downloadClick={downloadClick} />
		</tr>
	);
}

export default File;
