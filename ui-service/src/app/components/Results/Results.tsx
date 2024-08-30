"use client";
import React from "react";
import Loading from "../Loading/Loading";

interface ResultsProps {
  results: string;
  loading: boolean;
}
function Results({ results, loading }: ResultsProps) {
  return (
    <div className="w-96 h-96 overflow-auto">
      <h4 className="font-semibold text-2xl text-gray-900 ">Results:</h4>
      {loading ? <Loading /> : <p>  
        {results.split('\n').map((line, index) => (  
          <React.Fragment key={index}>  
            {line}  
            <br />  
          </React.Fragment>  
        ))}  
      </p>  }
    </div>
  );
}

export default Results;
