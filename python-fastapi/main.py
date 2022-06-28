import uvicorn
from fastapi import FastAPI
from fastapi.responses import PlainTextResponse

app = FastAPI()

@app.get("/ping", response_class=PlainTextResponse)
def ping():
    return "pong"

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=3000, log_level="critical")
