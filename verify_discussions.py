import requests
import json
import sys
import time

BASE_URL = "http://127.0.0.1:3000/api/v1"

def wait_for_server():
    print("Waiting for server to start...")
    for _ in range(30):
        try:
            requests.get(f"{BASE_URL}/repos")
            print("Server is up!")
            return
        except requests.exceptions.ConnectionError:
            time.sleep(1)
    print("Server failed to start")
    sys.exit(1)

def test_discussions():
    print("Verifying Discussion Endpoints...")
    repo_owner = "admin"
    repo_name = "codeza"

    try:
        # 1. Create Discussion
        print("\nTesting POST /repos/:owner/:repo/discussions...")
        payload = {
            "title": "Discussion Title",
            "body": "Discussion Body",
            "category": "Ideas"
        }
        r = requests.post(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/discussions", json=payload)
        r.raise_for_status()
        discussion = r.json()
        print(f"Created Discussion: {discussion['id']} - {discussion['title']}")
        disc_id = discussion['id']

        # 2. Get Discussion
        print(f"\nTesting GET /repos/:owner/:repo/discussions/{disc_id}...")
        r = requests.get(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/discussions/{disc_id}")
        r.raise_for_status()
        d = r.json()
        if d['id'] == disc_id and d['category'] == "Ideas":
             print("SUCCESS: Fetched discussion")
        else:
             print("FAILURE: Fetched wrong discussion")
             sys.exit(1)

        # 3. List Discussions
        print("\nTesting GET /repos/:owner/:repo/discussions...")
        r = requests.get(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/discussions")
        r.raise_for_status()
        discussions = r.json()
        found = any(d['id'] == disc_id for d in discussions)
        if found:
            print("SUCCESS: Discussion found in list")
        else:
            print("FAILURE: Discussion not found")
            sys.exit(1)

    except Exception as e:
        print(f"Error testing discussions: {e}")
        sys.exit(1)

if __name__ == "__main__":
    wait_for_server()
    test_discussions()
