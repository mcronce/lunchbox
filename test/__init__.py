import json
import requests

def req(session, method, url, **kwargs):
	print('>>>', method, url)
	if('headers' in kwargs):
		for (k, v) in kwargs['header'].items():
			print('%s: %s' % (k, v))
	if('json' in kwargs):
		print(json.dumps(kwargs['json'], indent = 4))
	response = session.request(method, url, **kwargs)
	print('<<<', response.status_code)
	try:
		response_json = response.json()
		print(json.dumps(response_json, indent = 4))
	except ValueError:
		pass
	print()
	return response

def get(session, url, **kwargs):
	return req(session, 'GET', url, **kwargs)

def post(session, url, **kwargs):
	return req(session, 'POST', url, **kwargs)

def delete(session, url, **kwargs):
	return req(session, 'DELETE', url, **kwargs)
	
