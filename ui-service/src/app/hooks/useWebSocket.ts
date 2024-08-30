import { useEffect, useRef, useCallback } from 'react';  

// Interfaces  
interface WebSocketMessage {  
  event: string;  
  message: string;  
}  

// Type Definitions  
type SetLoading = (loading: boolean) => void;  
type SetResult = React.Dispatch<React.SetStateAction<string>>;  

// Custom Hook  
const useWebSocket = (setLoading: SetLoading, setResult: SetResult) => {  
  const websocketRef = useRef<WebSocket | null>(null);  

  const initiateWebSocketConnection = useCallback(() => {  
    closeWebSocket();  

    const websocketUrl = createWebSocketUrl();  
    websocketRef.current = new WebSocket(websocketUrl);  

    websocketRef.current.onopen = handleOpen;  
    websocketRef.current.onmessage = handleMessage;  
  }, []);  

  const handleOpen = () => {  
    const connectData: WebSocketMessage = { event: 'connect', message: 'Connect' };  
    websocketRef.current?.send(JSON.stringify(connectData));  
  };  

  const handleMessage = async (event: MessageEvent) => {  
    setLoading(true);  

    const result = event.data?.trim() || '';  
    console.log(result);  

    setResult((prevResult) => prevResult + result + '\n');  
    setLoading(false);  
  };  

  const parseMessage = (data: string): WebSocketMessage => {  
    try {  
      return JSON.parse(data);  
    } catch (error) {  
      console.error('Failed to parse message', error);  
      return { event: '', message: '' };  
    }  
  };  

  const createWebSocketUrl = () => {  
    const host = window.location.hostname;  
    return `ws://${host}:8080/ws/result`;  
  };  

  const closeWebSocket = () => {  
    if (websocketRef.current) {  
      websocketRef.current.close();  
      websocketRef.current = null;  
    }  
  };  

  useEffect(() => {  
    initiateWebSocketConnection();  

    return () => closeWebSocket();  
  }, [initiateWebSocketConnection]);  

  return { websocket: websocketRef.current };  
};  

export default useWebSocket;