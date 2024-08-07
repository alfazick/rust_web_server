import requests
import json
import os
from dotenv import load_dotenv
import random

load_dotenv()

BASE_URL = os.getenv('BASE_URL')
GITHUB_TOKEN = os.getenv('GITHUB_TOKEN')

if not BASE_URL:
    raise ValueError("BASE_URL is not set in the .env file")

if not GITHUB_TOKEN:
    raise ValueError("GITHUB_TOKEN is not set in the .env file")

def make_request(method, endpoint, data=None, token=None):
    url = f"{BASE_URL}{endpoint}"
    headers = {
        "Authorization": f"Bearer {GITHUB_TOKEN}",
        "Accept": "application/json"
    }
    if token:
        headers["X-App-Token"] = token
    if data:
        headers["Content-Type"] = "application/json"
    
    response = getattr(requests, method)(url, json=data, headers=headers)
    
    print(f"{method.upper()} {endpoint} - Status: {response.status_code}")
    print("Request Headers:", headers)
    print("Request Data:", data)
    print("Response Headers:", response.headers)
    print("Response Content:", response.text)
    
    return response

def test_register():
    data = {
        "id": random.randint(1, 1000000),
        "username": "testuser",
        "password": "testpassword"
    }
    response = make_request('post', '/register', data)
    return response.status_code == 200

def test_login():
    data = {
        "id": random.randint(1, 1000000),
        "username": "testuser",
        "password": "testpassword"
    }
    response = make_request('post', '/login', data)
    if response.status_code == 200:
        return response.text.strip()  # Return the entire response as the token
    return None

def test_create_task(token):
    data = {
        "id": random.randint(1, 1000000),
        "name": "Test Task",
        "completed": False
    }
    response = make_request('post', '/task', data, token)
    if response.status_code == 200:
        print("Task created successfully.")
        return data['id']  # Return the ID we sent, since the server doesn't return it
    else:
        print(f"Failed to create task. Status code: {response.status_code}")
        return None

def test_read_all_tasks(token):
    response = make_request('get', '/task', token=token)
    if response.status_code == 200:
        try:
            return response.json()
        except json.JSONDecodeError:
            print("Could not decode JSON for reading all tasks.")
            print("Response content:", response.text)
    return None

def test_read_task(token, task_id):
    response = make_request('get', f'/task/{task_id}', token=token)
    if response.status_code == 200:
        try:
            return response.json()
        except json.JSONDecodeError:
            print(f"Could not decode JSON for reading task {task_id}.")
            print("Response content:", response.text)
    return None

def test_update_task(token, task_id):
    data = {
        "id": task_id,
        "name": "Updated Task",
        "completed": True
    }
    response = make_request('put', '/task', data, token)
    return response.status_code == 200

def test_delete_task(token, task_id):
    response = make_request('delete', f'/task/{task_id}', token=token)
    return response.status_code == 200

def main():
    print(f"Testing API at: {BASE_URL}")
    
    # Test register
    if test_register():
        print("Registration successful.")
    else:
        print("Registration failed.")
        return

    # Test login
    token = test_login()
    
    if token:
        print("Login successful. Token:", token)
        
        # Test create task
        task_id = test_create_task(token)
        
        if task_id:
            print(f"Task created with ID: {task_id}")
            
            # Test read all tasks
            tasks = test_read_all_tasks(token)
            if tasks:
                print("All tasks:", tasks)
            else:
                print("Failed to read all tasks.")
            
            # Test read specific task
            task = test_read_task(token, task_id)
            if task:
                print(f"Task {task_id}:", task)
            else:
                print(f"Failed to read task {task_id}.")
            
            # Test update task
            if test_update_task(token, task_id):
                print(f"Task {task_id} updated successfully.")
            else:
                print(f"Failed to update task {task_id}.")
            
            # Test delete task
            if test_delete_task(token, task_id):
                print(f"Task {task_id} deleted successfully.")
            else:
                print(f"Failed to delete task {task_id}.")
        else:
            print("Failed to create task.")
    else:
        print("Login failed. Unable to test task endpoints.")

if __name__ == "__main__":
    main()