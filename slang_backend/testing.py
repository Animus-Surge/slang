import requests

msgdata = {
    "message_author":1,
    "message_content":"Hewwo!",
    "message_sent":"2023-8-11/22:55:00"
}

req = requests.post("http://localhost:8000/groups/1/1/send", json=msgdata)
print(req.status_code)
print(req.text)