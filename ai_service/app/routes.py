import json
import requests
from fastapi import APIRouter, Body, Request, status
from fastapi.responses import JSONResponse

from app.config import OPENROUTER_API_KEY
from app.models import ChatRequestModel, ChatResponseModel

router = APIRouter()


@router.get('/health')
async def health_endpoint():
    return {"message": "Service is running!"}

@router.post('/chat', response_model=ChatResponseModel, status_code=status.HTTP_200_OK)
async def chat_endpoint(request: ChatRequestModel):
    
    # Make a POST request to OpenRouter API to generate a response.
    url = "https://openrouter.ai/api/v1/chat/completions"
    headers = {"Authorization": f"Bearer {OPENROUTER_API_KEY}"}
    msgs = [{'role': 'user', 'content': request.message}]

    body = {
        "model": "meta-llama/llama-3-70b-instruct:nitro",
        "messages": msgs,
        "temperature": 0,
    }
    
    try:   
        response = requests.post(
            url=url,
            headers=headers,
            data=json.dumps(body)
        )
        output = response.json()['choices'][0]['message']['content']
        output = output.strip().encode('utf-8')
        return {"result" : output}
    except Exception as e:
        return JSONResponse(
            status_code=400,
            content={"result": "Error occurred while processing the request. Please try again later."},
        )
    