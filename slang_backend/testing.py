import requests

msgdata = {
    
}

req = requests.post('', json=msgdata)
print(req.status_code)
print(req.text)