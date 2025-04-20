#!/bin/bash
echo curl http://127.0.0.1:8000/api/users 
curl http://127.0.0.1:8000/api/users 

echo curl http://127.0.0.1:8000/api/users/1
curl http://127.0.0.1:8000/api/users/1 

echo  curl -X POST -H "Content-Type: application/json" -d '{"name": "新しいユーザー"}' http://127.0.0.1:8000/api/users
curl -X POST -H "Content-Type: application/json" -d '{"name": "新しいユーザー"}' http://127.0.0.1:8000/api/users 

echo curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "name": "お休みRooちゃん", "active": false}' http://127.0.0.1:8000/api/users/1
curl -X PUT -H "Content-Type: application/json" -d '{"id": 1, "name": "お休みRooちゃん", "active": false}' http://127.0.0.1:8000/api/users/1 


echo curl -X DELETE http://127.0.0.1:8000/api/users/1
curl -X DELETE http://127.0.0.1:8000/api/users/1 
