import React, { useContext } from "react";
import "./Modal.css"; // Import your CSS file for modal styles
import { AppContext } from "./App";

function Modal(props) {
	const { isOpen, setIsOpen, setEncryptionKey, isDownloading, encryptionBool } =
		useContext(AppContext);

	const closeModal = () => {
		setIsOpen(false);
	};

	return (
		<div>
			{isOpen && (
				<div className="modal">
					<div className="modal-content">
						<span className="close" onClick={closeModal}>
							&times;
						</span>
						<h2 className="title">File Download</h2>

						{encryptionBool && (
							<div>
								<h3>
									<label htmlFor="encryption-key">Key</label>
									<input
										type="text"
										id="encryption-key"
										onChange={(e) => setEncryptionKey(e.target.value)}
									/>
								</h3>
							</div>
						)}

						<button onClick={props.downloadClick} className="download-button">
							{isDownloading ? "Processing..." : "Download File"}
						</button>
					</div>
				</div>
			)}
		</div>
	);
}

export default Modal;
