import React from "react";

interface TextInputProps {
  setInputText: React.Dispatch<React.SetStateAction<string>>;
}
function TextInput({ setInputText }: TextInputProps) {
  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputText(e.target.value);
  };

  return (
    <textarea
      onChange={handleChange}
      className="w-96 border border-gray-400 p-4 rounded-lg h-96"
      placeholder="Enter your text here..."
    ></textarea>
  );
}

export default TextInput;
