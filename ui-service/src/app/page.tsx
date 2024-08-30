"use client";
import TextInput from "./components/TextInput/TextInput";
import CtaButton from "./components/Button/CtaButton";
import Results from "./components/Results/Results";
import axios from "axios";
import { toast } from "react-toastify";
import { useState } from "react";
import useWebSocket from "./hooks/useWebSocket";

interface SummType {
  name: string;
}

export default function Home() {
  const [inputText, setInputText] = useState<string>("");
  const [results, setResults] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const { websocket } = useWebSocket(setLoading, setResults);

  const handleSummarize = async () => {
    
    try {
      const response = await axios.post("/api/subscribe", { text: inputText });
      if (response.status === 200) {
        toast.success("Subscription is successfully done!");
      } else {
        toast.error("Failed to subscribe!");
      }
    } catch (error) {
      console.log('Summarize error:', error);
      toast.error("Failed to subscribe!");
    }
  };

  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <h2 className="font-black text-4xl text-gray-900">
        <span className="text-orange-500">Assignment</span> - Distributed AI Application
      </h2>
      <div className="flex mt-8 gap-8">
        <TextInput setInputText={setInputText} />
        <Results results={results} loading={loading} />
      </div>
      <CtaButton onClick={() => handleSummarize()} />
    </main>
  );
}
