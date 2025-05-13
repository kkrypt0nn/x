FROM python:alpine

WORKDIR /app

COPY app/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY app/ .
EXPOSE 8080

CMD ["python", "main.py"]
