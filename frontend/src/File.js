import React from "react";
function fileInfo(props) {
	return (
		<div>{`File Size: ${props.fileSize} File Name: ${props.fileName}`}</div>
	);
}

export default fileInfo;
