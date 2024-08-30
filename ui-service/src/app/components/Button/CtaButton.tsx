import React from "react";

interface ButtonProps {
  onClick: any;
}
function CtaButton({ onClick }: ButtonProps) {
  return (
    <button
      onClick={() => onClick()}
      className="px-4 py-2 rounded-lg bg-orange-500 text-white hover:bg-orange-600 mt-4"
    >
      Subscribe
    </button>
  );
}

export default CtaButton;
