import requests

data = {
    "title":"Test title",
    "body":"aliebfaiubfla iuefhaliewufbal iuwehfliuawef uiawehfa ilwuef iwebf auwf lauhef uvhe Animus_Surge, awienfa uafe uwae"
}

req = requests.post("http://localhost:8000/posts/create", json=data)
print(req.status_code)