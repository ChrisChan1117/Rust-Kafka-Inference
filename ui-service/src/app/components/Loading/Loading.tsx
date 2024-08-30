"use client";
import React from "react";
import { CSSProperties } from "react";
import { BeatLoader } from "react-spinners";
const override: CSSProperties = {
  display: "block",
  margin: "0 auto",
  borderColor: "red",
};
function Loading() {
  return (
    <div className="relative h-full">
      <BeatLoader
        color="#f97316"
        loading={true}
        cssOverride={override}
        size={15}
        aria-label="Loading Spinner"
        data-testid="loader"
        className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
      />
    </div>
  );
}
export default Loading;
