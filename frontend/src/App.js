import "./App.css";
import { createContext, useEffect, useState } from "react";
import File from "./File";
import Upload from "./Upload";

export const AppContext = createContext();
function App() {
	const [fileInfo, setFileInfo] = useState({});
	const [isDownloading, setIsDownloading] = useState(false);
	const [isDeleting, setIsDeleting] = useState(false);
	const [filePath, setFilePath] = useState(""); // Replace with your actual file path
	const [isOpen, setIsOpen] = useState(false);
	const defaultFilename = "my_downloaded_file.txt"; // Optional default filename
	const [selectedFile, setSelectedFile] = useState(null);
	const [encryptionBool, setEncryptionBool] = useState(false);
	const [encryptionKey, setEncryptionKey] = useState("");
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
			setFileInfo(data);
		} catch (error) {
			console.error("Error fetching file list:", error);
			// Handle fetch error (e.g., display error message)
		}
	};

	return (
		<div className="container">
			<AppContext.Provider
				value={{
					fileInfo,
					setFileInfo,
					isDownloading,
					setIsDownloading,
					isDeleting,
					setIsDeleting,
					filePath,
					setFilePath,
					isOpen,
					setIsOpen,
					defaultFilename,
					setSelectedFile,
					selectedFile,
					encryptionBool,
					setEncryptionBool,
					encryptionKey,
					setEncryptionKey,
				}}
			>
				<header className="header">
					<h1>CloudCord File Manager</h1>
					<div>
						<Upload />
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
						{fileInfo && fileInfo.names && fileInfo.names.length > 0 ? (
							fileInfo.names.map((fileName, key) => (
								<File
									fileName={fileName}
									encrypted={fileInfo.encryptions[key]} // Access the corresponding encryption using the key
								/>
							))
						) : (
							<tr>
								<td colSpan="2">No files uploaded</td>
							</tr>
						)}
					</tbody>
				</table>
			</AppContext.Provider>
		</div>
	);
}

export default App;
