import requests

from . import *

session = requests.Session()

# No session cookie should return a Bad Request
response = post(session, 'http://localhost:8080/api/authorize', json = {'email' : 'mike@cronce.io', 'password' : 'faster'})
assert(response.status_code == 400)

# Wrong/missing params should all return Bad Request
response = post(session, 'http://localhost:8080/api/authorize', json = {'emali' : 'mike@cronce.io', 'password' : 'faster'})
assert(response.status_code == 400)
response = post(session, 'http://localhost:8080/api/authorize', json = {'email' : 'mike@cronce.io'})
assert(response.status_code == 400)

# Wrong password should return Unauthorized
response = post(session, 'http://localhost:8080/api/authorize', json = {'email' : 'mike@cronce.io', 'password' : 'herpderp'})
assert(response.status_code == 401)
# Wrong email should return Unauthorized
response = post(session ,'http://localhost:8080/api/authorize', json = {'email' : 'herp@de.rp', 'password' : 'faster'})
assert(response.status_code == 401)

# Not authenticated; should return Unauthorized 
response = get(session, 'http://localhost:8080/api/provider/providers')
assert(response.status_code == 401)
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'id' : 2, 'email' : 'abc@d.e', 'password' : 'derp'})
assert(response.status_code == 401)

# Finally, correct username/password with established session cookie should return OK and 'true'
response = post(session, 'http://localhost:8080/api/authorize', json = {'email' : 'mike@cronce.io', 'password' : 'faster'})
assert(response.status_code == 200)
assert(response.json() == True)

# Missing any required field should return Bad Request
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'id' : 2})
assert(response.status_code == 400)
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'email' : 'abc@d.e'})
assert(response.status_code == 400)
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'password' : 'derp'})
assert(response.status_code == 400)
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'id' : 2, 'email' : 'abc@d.e'})
assert(response.status_code == 400)
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'id' : 2, 'password' : 'derp'})
assert(response.status_code == 400)
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'email' : 'abc@d.e', 'password' : 'derp'})
assert(response.status_code == 400)

# Finally, being logged in and sending all fields should return OK
response = post(session, 'http://localhost:8080/api/provider/providers', json = {'id' : 2, 'email' : 'abc@d.e', 'password' : 'derp'})
assert(response.status_code == 200)

# Being logged in should return OK and an array of 3 items
# TODO:  Validate contents of the array
response = get(session, 'http://localhost:8080/api/provider/providers')
assert(response.status_code == 200)
assert(type(response.json()) == list)
assert(len(response.json()) == 3)

