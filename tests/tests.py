import requests


url = "http://127.0.0.1:8010/subscribe"


response = requests.post(url,data={"name":"Tom","email":"123456@qq.com"})

print(response.json)