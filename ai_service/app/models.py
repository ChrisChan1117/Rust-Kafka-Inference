from pydantic import BaseModel  

class ChatResponseModel(BaseModel):  
    result: str  

class ChatRequestModel(BaseModel):  
    message: str  